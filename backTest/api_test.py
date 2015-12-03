import unittest
from mock import patch
import json
import copy

from google.appengine.ext import ndb
from webtest import TestApp

import api
import fixtures
import models

app = TestApp(api.app)


class CreateHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    def test_should_be_able_to_create_game(self):
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

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
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(fixtures.SETUP),
            'join_hash': game.join_hash
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(fixtures.DEFAULT_GAME)

        self.assertEqual(game.get_board(), current_state_of_game)


class MoveHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_move(self, pusher):
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(fixtures.SETUP),
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 6}),
            'to': json.dumps({'x': 5, 'y': 5})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(fixtures.DEFAULT_GAME)

        current_state_of_game[5][5] = {'side': 0, 'rank': '4'}
        current_state_of_game[6][5] = 0

        self.assertEqual(game.get_board(), current_state_of_game)

        # Blue's turn
        self.assertEqual(game.turn, 1)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_attack_and_draw(self, pusher):
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(fixtures.SETUP),
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
            'from': json.dumps({'x': 5, 'y': 3}),
            'to': json.dumps({'x': 5, 'y': 4})
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 5}),
            'to': json.dumps({'x': 5, 'y': 4})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(fixtures.DEFAULT_GAME)

        # These pieces should have been destroyed
        current_state_of_game[3][5] = 0
        current_state_of_game[6][5] = 0

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(json.loads(game.last_move), {
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
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(fixtures.SETUP),
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
            'from': json.dumps({'x': 0, 'y': 3}),
            'to': json.dumps({'x': 0, 'y': 4})
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
            'from': json.dumps({'x': 0, 'y': 4}),
            'to': json.dumps({'x': 1, 'y': 4})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(fixtures.DEFAULT_GAME)

        current_state_of_game[6][1] = 0
        current_state_of_game[3][0] = 0

        current_state_of_game[4][1] = {'side': 1, 'rank': '1'}
        current_state_of_game[4][0] = 0

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(json.loads(game.last_move), {
            'type': 'won',
            'from': {
                'piece': {'side': 1, 'rank': '1'},
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
    def test_should_be_able_to_attack_and_lose(self, pusher):
        app.post('/api/create', params={'board': json.dumps(fixtures.SETUP)})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': json.dumps(fixtures.SETUP),
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
            'from': json.dumps({'x': 0, 'y': 3}),
            'to': json.dumps({'x': 0, 'y': 4})
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
            'from': json.dumps({'x': 4, 'y': 3}),
            'to': json.dumps({'x': 4, 'y': 4})
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 1, 'y': 4}),
            'to': json.dumps({'x': 0, 'y': 4})
        })

        game = models.Game.query().get()

        current_state_of_game = copy.deepcopy(fixtures.DEFAULT_GAME)

        current_state_of_game[6][1] = 0
        current_state_of_game[3][0] = 0

        current_state_of_game[3][4] = 0
        current_state_of_game[4][4] = {'side': 1, 'rank': '4'}

        current_state_of_game[4][0] = {'side': 1, 'rank': '1'}

        self.assertEqual(game.get_board(), current_state_of_game)

        self.assertEqual(json.loads(game.last_move), {
            'type': 'lost',
            'from': {
                'piece': {'side': 0, 'rank': '2'},
                'position': {'x': 1, 'y': 4}
            },
            'to': {
                'piece': {'side': 1, 'rank': '1'},
                'position': {'x': 0, 'y': 4}
            }
        })

        # Blue's turn
        self.assertEqual(game.turn, 1)
