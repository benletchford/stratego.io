import json
import webapp2
import uuid

from lib import ndb_json
from lib.pusher.pusher import Pusher

from utils import status_codes, general, board_utils, pusher_utils

import models


class CreateHandler(webapp2.RequestHandler):

    def post(self):
        json_board = self.request.get('board')
        board = json.loads(json_board)

        for row in board:
            for piece in row:
                piece['side'] = 0

        new_game = models.Game()
        new_game.red_hash = uuid.uuid4().hex[:6]
        new_game.blue_hash = uuid.uuid4().hex[:6]
        new_game.join_hash = uuid.uuid4().hex[:6]

        new_game.set_red_setup(board)

        # Set the water.
        new_game.set_blocks()

        new_game.put()

        game_dict = board_utils.get_sendable_game(new_game, 0)

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
            game.put()

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
        fromPos = json.loads(self.request.get('from'))
        toPos = json.loads(self.request.get('to'))

        if side == 0:
            game = models.Game.query(
                models.Game.red_hash == player_hash
            ).get()

        elif side == 1:
            game = models.Game.query(
                models.Game.blue_hash == player_hash
            ).get()

        try:
            # Will raise if not valid.
            game.check_move(fromPos, toPos)

            game.move_piece(fromPos, toPos)

            game.put()

            # Send move to opponent
            pusher = Pusher(app_id=pusher_utils.APP_ID,
                            key=pusher_utils.KEY,
                            secret=pusher_utils.SECRET)

            pusher.trigger(u'game-%s' % game.get_opponent_hash(player_hash),
                           u'move',
                           {u'from': fromPos,
                            u'to': toPos})

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


app = webapp2.WSGIApplication([
    ('/api/create', CreateHandler),
    ('/api/join', JoinHandler),
    ('/api/move', MoveHandler),
    ('/api/game', GameHandler),
], debug=True)
