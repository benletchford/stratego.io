import json
import webapp2
import uuid

from lib import ndb_json
from lib.pusher.pusher import Pusher

from utils import status_codes, general

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
        new_game.put()

        response = {
            'red_hash': new_game.red_hash
        }

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(response))


class JoinHandler(webapp2.RequestHandler):

    def post(self):
        board = self.request.get('board')


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

        game.move(fromPos, toPos)
        game.put()


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

        game_dict = json.loads(ndb_json.dumps(game))

        # These are secret and should never be sent.
        del game_dict['red_hash']
        del game_dict['blue_hash']
        del game_dict['join_hash']
        del game_dict['red_setup']
        del game_dict['blue_setup']

        game_dict['side'] = side

        # We know the board is in json, let's load it so everything is on one
        # level and not wrapped in a string.
        game_dict['board'] = game.get_board()

        if side == 0:
            if not game.blue_setup:
                unknown = {'rank': '?', 'side': 1}
                unknown_array = [unknown, unknown, unknown, unknown,
                                 unknown, unknown, unknown, unknown, unknown, unknown]

                game_dict['board'][0] = unknown_array
                game_dict['board'][1] = unknown_array
                game_dict['board'][2] = unknown_array
                game_dict['board'][3] = unknown_array

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(game_dict))


app = webapp2.WSGIApplication([
    ('/api/create', CreateHandler),
    ('/api/join', JoinHandler),
    ('/api/move', MoveHandler),
    ('/api/game', GameHandler),
], debug=True)
