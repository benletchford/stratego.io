import json
import webapp2

from lib import ndb_json
from lib.pusher.pusher import Pusher

from utils import status_codes

import models


class CreatePrivateHandler(webapp2.RequestHandler):

    def post(self):
        board_setup = self.request.get('board_setup')


class CreatePublicHandler(webapp2.RequestHandler):

    def post(self):
        board_setup = self.request.get('board_setup')


class JoinPrivateHandler(webapp2.RequestHandler):

    def post(self):
        board_setup = self.request.get('board_setup')


class JoinPublicHandler(webapp2.RequestHandler):

    def post(self):
        board_setup = self.request.get('board_setup')


class MoveHandler(webapp2.RequestHandler):

    def post(self):
        board_setup = self.request.get('board_setup')


class GameHandler(webapp2.RequestHandler):

    def get(self):
        player_hash = self.request.get('player_hash')

        if not player_hash:
            self.response.set_status(status_codes.UNAUTHORIZED)
            return

        a_game = models.Game()
        a_game.put()

        game = models.Game.get_by_id(a_game.key.id())

        game = json.loads(ndb_json.dumps(game))

        # These are secret and should never be sent.
        del game['red_hash']
        del game['blue_hash']
        del game['join_hash']

        # We know the board is in json, let's load it so everything is on one
        # level and not wrapped in a string.
        game['board'] = json.loads(game['board'])

        self.response.headers['Content-Type'] = 'text/json'
        self.response.write(json.dumps(game))


app = webapp2.WSGIApplication([
    ('/api/create/private', CreatePrivateHandler),
    ('/api/create/public', CreatePublicHandler),

    ('/api/join/private', JoinPrivateHandler),
    ('/api/join/public', JoinPublicHandler),

    ('/api/move', MoveHandler),
    ('/api/game', GameHandler),
], debug=True)
