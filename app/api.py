import json
import webapp2
import uuid

from google.appengine.api import taskqueue

from lib import ndb_json
from lib.pusher.pusher import Pusher

from utils import status_codes, general, board_utils, pusher_utils

import models
import move_types
import game_states


def _create_game(setup):
    for row in setup:
        for piece in row:
            piece['side'] = 0

    new_game = models.Game()
    new_game.red_hash = uuid.uuid4().hex[:6]
    new_game.blue_hash = uuid.uuid4().hex[:6]
    new_game.join_hash = uuid.uuid4().hex[:6]

    new_game.set_red_setup(setup)

    # Set the water.
    new_game.set_blocks()

    new_game.put()

    return new_game


def _join_game(setup, join_hash, game=None):
    for row in setup:
        for piece in row:
            piece['side'] = 1

    if not game:
        game = models.Game.query(
            models.Game.join_hash == join_hash
        ).get()

    if game:
        game.set_blue_setup(setup)
        game.game_state = game_states.READY
        game.put()

    else:
        raise Exception("Can't find that game to join.")

    return game


class CreateHandler(webapp2.RequestHandler):

    def post(self):
        json_board = self.request.get('board')
        board = json.loads(json_board)

        game_dict = board_utils.get_sendable_game(_create_game(board), 0)

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(game_dict))


class JoinHandler(webapp2.RequestHandler):

    def post(self):
        if not general.array_has_values(self.request.arguments(), ['join_hash', 'board']):
            self.response.set_status(status_codes.INTERNAL_ERROR)
            return

        join_hash = self.request.get('join_hash')
        board = json.loads(self.request.get('board'))

        for row in board:
            for piece in row:
                piece['side'] = 1

        game = models.Game.query(
            models.Game.join_hash == join_hash
        ).get()

        if game:
            game.set_blue_setup(board)
            game.game_state = game_states.READY
            game.put()

            # Tell red we're ready.
            pusher = Pusher(app_id=pusher_utils.APP_ID,
                            key=pusher_utils.KEY,
                            secret=pusher_utils.SECRET)

            pusher.trigger('public-game-%s' % game.red_hash,
                           'blue_ready',
                           {})

            game_dict = board_utils.get_sendable_game(game, 1)

            self.response.headers['Content-Type'] = 'text/json'
            self.response.write(json.dumps(game_dict))

        else:
            self.response.set_status(status_codes.NOT_FOUND)
            return


class MoveHandler(webapp2.RequestHandler):

    def post(self):
        if not general.array_has_values(self.request.arguments(), ['player_hash', 'side', 'from', 'to']):
            self.response.set_status(status_codes.INTERNAL_ERROR)
            return

        player_hash = self.request.get('player_hash')
        side = int(self.request.get('side'))
        from_pos = json.loads(self.request.get('from'))
        to_pos = json.loads(self.request.get('to'))

        if side == 0:
            game = models.Game.query(
                models.Game.red_hash == player_hash
            ).get()

        elif side == 1:
            game = models.Game.query(
                models.Game.blue_hash == player_hash
            ).get()

        if game.has_ended():
            self.response.set_status(status_codes.UNAUTHORIZED)
            return

        try:
            # Will raise if not valid.
            move_type = game.check_move(from_pos, to_pos)

            if move_type == move_types.MOVE:
                game.move_piece(from_pos, to_pos)
                game.flip_turn()
                game.set_last_move({
                    'type': 'move',
                    'from': {
                        'position': from_pos
                    },
                    'to': {
                        'position': to_pos
                    }
                })

            elif move_type == move_types.ATTACK_WON:
                from_piece = game.get_piece(from_pos)
                to_piece = game.get_piece(to_pos)

                game.move_piece(from_pos, to_pos)

                game.flip_turn()
                game.set_last_move({
                    'type': 'won',
                    'from': {
                        'piece': from_piece,
                        'position': from_pos
                    },
                    'to': {
                        'piece': to_piece,
                        'position': to_pos
                    }
                })

            elif move_type == move_types.ATTACK_LOST:
                from_piece = game.get_piece(from_pos)
                to_piece = game.get_piece(to_pos)

                game.delete_piece(from_pos)

                game.flip_turn()
                game.set_last_move({
                    'type': 'lost',
                    'from': {
                        'piece': from_piece,
                        'position': from_pos
                    },
                    'to': {
                        'piece': to_piece,
                        'position': to_pos
                    }
                })

            elif move_type == move_types.ATTACK_DRAW:
                from_piece = game.get_piece(from_pos)
                to_piece = game.get_piece(to_pos)

                game.delete_piece(from_pos)
                game.delete_piece(to_pos)

                game.flip_turn()
                game.set_last_move({
                    'type': 'draw',
                    'from': {
                        'piece': from_piece,
                        'position': from_pos
                    },
                    'to': {
                        'piece': to_piece,
                        'position': to_pos
                    }
                })

            elif move_type == move_types.CAPTURE:
                from_piece = game.get_piece(from_pos)
                to_piece = game.get_piece(to_pos)

                game.move_piece(from_pos, to_pos)

                game.set_last_move({
                    'type': 'capture',
                    'from': {
                        'piece': from_piece,
                        'position': from_pos
                    },
                    'to': {
                        'piece': to_piece,
                        'position': to_pos
                    }
                })

            game.put()

            # Tell clients to update
            pusher = Pusher(app_id=pusher_utils.APP_ID,
                            key=pusher_utils.KEY,
                            secret=pusher_utils.SECRET)

            pusher.trigger('public-game-%s' % game.get_opponent_hash(player_hash),
                           'update',
                           {'command': 'refresh'})

            game_dict = board_utils.get_sendable_game(game, side)

            self.response.headers['Content-Type'] = 'text/json'
            self.response.write(json.dumps(game_dict))

        except models.InvalidMove:
            self.response.set_status(status_codes.UNAUTHORIZED)


class GameHandler(webapp2.RequestHandler):

    def get(self):
        player_hash = self.request.get('player_hash')

        if not player_hash:
            self.response.set_status(status_codes.UNAUTHORIZED)
            return

        game = models.Game.query(
            models.Game.red_hash == player_hash
        ).get()
        side = 0

        if not game:
            game = models.Game.query(
                models.Game.blue_hash == player_hash
            ).get()
            side = 1

        # If still not ;)
        if not game:
            self.response.set_status(status_codes.NOT_FOUND)
            return

        game_dict = board_utils.get_sendable_game(game, side)

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(game_dict))


class JoinPoolHandler(webapp2.RequestHandler):

    def post(self):
        if not general.array_has_values(self.request.arguments(), ['board', 'socket_id']):
            self.response.set_status(status_codes.INTERNAL_ERROR)
            return

        board = self.request.get('board')
        socket_id = self.request.get('socket_id')

        params = {
            'setup': board,
            'socket_id': socket_id
        }

        q = taskqueue.Queue('pool')
        q.add(
            taskqueue.Task(url='/api/pool/process', params=params, method='post'))

        self.response.set_status(200)


class ProcessPoolHandler(webapp2.RequestHandler):

    def post(self):
        setup = self.request.get('setup')
        socket_id = self.request.get('socket_id')

        pusher = Pusher(app_id=pusher_utils.APP_ID,
                        key=pusher_utils.KEY,
                        secret=pusher_utils.SECRET)

        # Get the oldest...
        oldest_game = models.Pool.query().order(-models.Pool.created).get()

        if oldest_game:
            oldest_game_channel_info = pusher.channel_info(
                'public-pool-%s' % oldest_game.socket_id, ['occupied'])

            # Is red still here?
            if oldest_game_channel_info['occupied']:
                task_game_channel_info = pusher.channel_info(
                    'public-pool-%s' % socket_id, ['occupied'])

                # Is blue still here?
                if task_game_channel_info['occupied']:
                    # We connect these two guys and create a game with the
                    # oldest game as red.
                    red_game = _create_game(json.loads(oldest_game.setup))

                    blue_game = _join_game(json.loads(setup),
                                           red_game.join_hash,
                                           red_game)

                    pusher.trigger('public-pool-%s' % oldest_game.socket_id,
                                   'opponent_found',
                                   {'player_hash': red_game.red_hash})

                    pusher.trigger('public-pool-%s' % socket_id,
                                   'opponent_found',
                                   {'player_hash': blue_game.blue_hash})

                # Blue's not here, remove the task
                else:
                    self.response.set_status(status_codes.OK)
                    return

            # We fail
            else:
                self.response.set_status(status_codes.NOT_FOUND)

            # Delete the oldest game as it should no longer be in the pool.
            oldest_game.key.delete()

        # We become the host for the next guy...
        else:
            models.Pool(
                setup=setup,
                socket_id=socket_id
            ).put()

            self.response.set_status(status_codes.OK)


class PusherAuthHandler(webapp2.RequestHandler):

    def post(self):
        if not general.array_has_values(self.request.arguments(), ['channel_name', 'socket_id']):
            self.response.set_status(status_codes.INTERNAL_ERROR)
            return

        channel_name = self.request.get('channel_name')
        socket_id = self.request.get('socket_id')

        pusher = Pusher(app_id=pusher_utils.APP_ID,
                        key=pusher_utils.KEY,
                        secret=pusher_utils.SECRET)

        auth = pusher.authenticate(
            channel=channel_name,
            socket_id=socket_id,
            custom_data={}
        )

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(auth))


app = webapp2.WSGIApplication([
    ('/api/create', CreateHandler),
    ('/api/join', JoinHandler),
    ('/api/move', MoveHandler),
    ('/api/game', GameHandler),
    ('/api/pool/join', JoinPoolHandler),
    ('/api/pool/process', ProcessPoolHandler),
    ('/api/pusher/auth', PusherAuthHandler)
], debug=True)
