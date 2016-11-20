import unittest
from mock import patch
import json
import copy

from google.appengine.ext import ndb
from webtest import TestApp

import api
import FIXTURES
import models
from utils import board_utils
from CONSTANTS import STATUS_CODES


app = TestApp(api.app)


class CreateHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    def test_should_be_able_to_create_game(self):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        self.assertEqual(game.get_board(), [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [
                {'side': 0, 'rank': '1'},
                {'side': 0, 'rank': '2'},
                {'side': 0, 'rank': '3'},
                {'side': 0, 'rank': '3'},
                {'side': 0, 'rank': '4'},
                {'side': 0, 'rank': '4'},
                {'side': 0, 'rank': '4'},
                {'side': 0, 'rank': '5'},
                {'side': 0, 'rank': '5'},
                {'side': 0, 'rank': '5'}
            ],
            [
                {'side': 0, 'rank': '5'},
                {'side': 0, 'rank': '6'},
                {'side': 0, 'rank': '6'},
                {'side': 0, 'rank': '6'},
                {'side': 0, 'rank': '6'},
                {'side': 0, 'rank': '7'},
                {'side': 0, 'rank': '7'},
                {'side': 0, 'rank': '7'},
                {'side': 0, 'rank': '7'},
                {'side': 0, 'rank': '8'}
            ],
            [
                {'side': 0, 'rank': '8'},
                {'side': 0, 'rank': '8'},
                {'side': 0, 'rank': '8'},
                {'side': 0, 'rank': '8'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'}
            ],
            [
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': '9'},
                {'side': 0, 'rank': 'S'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'B'},
                {'side': 0, 'rank': 'F'}
            ]
        ])

        self.assertEqual(len(game.red_hash), 6)
        self.assertEqual(len(game.blue_hash), 6)
        self.assertEqual(len(game.join_hash), 6)

        # Make sure it is red's turn.
        self.assertEqual(game.turn, False)


class JoinHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_join_game(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(FIXTURES.DEFAULT_GAME)

        self.assertEqual(game.get_board(), current_state_of_game)


class MoveHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_move(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 6}),
            'to': json.dumps({'x': 5, 'y': 5})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(FIXTURES.DEFAULT_GAME)

        current_state_of_game[5][5] = {'side': 0, 'rank': '4'}
        current_state_of_game[6][5] = 0

        self.assertEqual(game.get_board(), current_state_of_game)

        # Blue's turn
        self.assertEqual(game.turn, 1)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_attack_and_draw(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 6}),
            'to': json.dumps({'x': 5, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 4, 'y': 6}),
            'to': json.dumps({'x': 4, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 5}),
            'to': json.dumps({'x': 5, 'y': 4})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(FIXTURES.DEFAULT_GAME)

        # These pieces should have been destroyed
        current_state_of_game[3][5] = 0
        current_state_of_game[6][5] = 0

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(game.get_last_move(), {
            'type': 'draw',
            'from': {
                'piece': {'side': 0, 'rank': '4'},
                'position': {'x': 5, 'y': 5}
            },
            'to': {
                'piece': {'side': 1, 'rank': '4'},
                'position': {'x': 5, 'y': 4}
            }
        })

        # Blue's turn
        self.assertEqual(game.turn, 1)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_attack_and_win(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 6}),
            'to': json.dumps({'x': 1, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 8, 'y': 6}),
            'to': json.dumps({'x': 8, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 5}),
            'to': json.dumps({'x': 1, 'y': 4})
        })

        # app.post('/api/move', params={
        #     'player_hash': game.blue_hash,
        #     'side': 1,
        #     'from': json.dumps({'x': 0, 'y': 4}),
        #     'to': json.dumps({'x': 1, 'y': 4})
        # })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(FIXTURES.DEFAULT_GAME)

        current_state_of_game[3][1] = 0
        current_state_of_game[6][1] = 0
        current_state_of_game[4][1] = {'side': 0, 'rank': '2'}

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(game.get_last_move(), {
            'type': 'won',
            'from': {
                'piece': {'side': 0, 'rank': '2'},
                'position': {'x': 1, 'y': 5}
            },
            'to': {
                'piece': {'side': 1, 'rank': '5'},
                'position': {'x': 1, 'y': 4}
            }
        })

        # Blue's turn
        self.assertEqual(game.turn, 1)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_attack_and_lose(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 6}),
            'to': json.dumps({'x': 1, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 6}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 5}),
            'to': json.dumps({'x': 1, 'y': 4})
        })

        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 8, 'y': 5})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(FIXTURES.DEFAULT_GAME)

        current_state_of_game[6][1] = 0
        current_state_of_game[3][0] = 0
        current_state_of_game[4][1] = {'side': 0, 'rank': '2'}

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(game.get_last_move(), {
            'type': 'lost',
            'from': {
                'piece': {'side': 1, 'rank': '5'},
                'position': {'x': 0, 'y': 4}
            },
            'to': {
                'piece': {'side': 0, 'rank': '2'},
                'position': {'x': 1, 'y': 4}
            }
        })

        # Red's turn
        self.assertEqual(game.turn, 0)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_violate_two_square_rule(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        # Red 1st
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 0, 'y': 6}),
            'to': json.dumps({'x': 0, 'y': 5})
        })

        # Blue 1st
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 6}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        # Red 2nd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 0, 'y': 5}),
            'to': json.dumps({'x': 0, 'y': 6})
        })

        # Blue 2nd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 6})
        })

        # Red 3rd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 0, 'y': 6}),
            'to': json.dumps({'x': 0, 'y': 5})
        })

        # Blue 3rd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 6}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        game = models.Game.query().get()

        self.assertEqual(len(game.get_moves()), 6)

        # Red tries to break two-square rule
        response = app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 0, 'y': 5}),
            'to': json.dumps({'x': 0, 'y': 6})
        }, expect_errors=True)

        self.assertEqual(response.status_code, STATUS_CODES.UNAUTHORIZED)
        self.assertEqual(response.json_body['message'],
            'That move violates the two-square rule.'
        )

        # Red 4th
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 6}),
            'to': json.dumps({'x': 1, 'y': 5})
        })

        self.assertEqual(len(game.get_moves()), 7)
        # Blue's turn
        self.assertEqual(game.turn, True)

        # Blue tries to break two-square rule
        response = app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 6})
        }, expect_errors=True)
        self.assertEqual(response.status_code, STATUS_CODES.UNAUTHORIZED)
        self.assertEqual(response.json_body['message'],
            'That move violates the two-square rule.'
        )

        self.assertEqual(len(game.get_moves()), 7)
        # Still Blue's turn
        self.assertEqual(game.turn, True)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_scout_violate_two_square_rule(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        game.set_board([
            [{'rank': '9', 'side': 1}, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, {'rank': '9', 'side': 0}],
        ])
        game.put()

        # Red's 1st
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 9}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        # Blue's 1st
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 9}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        # Red's 2nd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 9})
        })

        # Blue's 2nd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 9})
        })

        # Red's 3rd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 9}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        # Blue's 3rd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 9}),
            'to': json.dumps({'x': 9, 'y': 5})
        })

        # Red tries to break two-square rule
        response = app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 8})
        }, expect_errors=True)
        self.assertEqual(response.status_code, STATUS_CODES.UNAUTHORIZED)
        self.assertEqual(response.json_body['message'],
            'That move violates the two-square rule.'
        )

        # Red tries to break two-square rule
        response = app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 6})
        }, expect_errors=True)
        self.assertEqual(response.status_code, STATUS_CODES.UNAUTHORIZED)
        self.assertEqual(response.json_body['message'],
            'That move violates the two-square rule.'
        )

        response = app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 9, 'y': 0})
        })

        # Different axis shouldn't trigger 2S.
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 5}),
            'to': json.dumps({'x': 0, 'y': 5})
        })

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_scout_violate_two_square_rule_with_two_different_pieces(self, pusher):
        app.post('/api/create', params={'board': json.dumps(FIXTURES.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(FIXTURES.SETUP),
            'join_hash': game.join_hash
        })

        game.set_board([
            [{'rank': '9', 'side': 1}, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [{'rank': '9', 'side': 1}, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, {'rank': '9', 'side': 0}],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, {'rank': '9', 'side': 0}],
        ])
        game.put()

        # Red's 1st
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 8}),
            'to': json.dumps({'x': 9, 'y': 3})
        })

        # Blue's 1st
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 8}),
            'to': json.dumps({'x': 9, 'y': 3})
        })

        # Red's 2nd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 3}),
            'to': json.dumps({'x': 9, 'y': 8})
        })

        # Blue's 2nd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 3}),
            'to': json.dumps({'x': 9, 'y': 8})
        })

        # Red's 3rd
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 8}),
            'to': json.dumps({'x': 9, 'y': 3})
        })

        # Blue's 3rd
        app.post('/api/move', params={
            'player_hash': game.blue_hash,
            'side': 1,
            'from': json.dumps({'x': 9, 'y': 8}),
            'to': json.dumps({'x': 9, 'y': 3})
        })

        # Red's 4th move with different piece - shouldn't break 2S rule.
        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 9, 'y': 9}),
            'to': json.dumps({'x': 9, 'y': 4})
        })
