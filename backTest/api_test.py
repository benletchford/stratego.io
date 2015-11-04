import unittest
from mock import patch
import json

from google.appengine.ext import ndb
from webtest import TestApp

import api
import fixtures
import models

app = TestApp(api.app)


class CreateHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    def test_should_be_able_to_create_game(self):
        app.post('/api/create', params={'board': fixtures.SETUP})

        game = models.Game.query().get()

        self.assertEqual(game.get_board(), [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [
                {u'side': 0, u'rank': u'1'},
                {u'side': 0, u'rank': u'2'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'}
            ],
            [
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'8'}
            ],
            [
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'}
            ],
            [
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'S'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'F'}
            ]
        ])

        self.assertEqual(len(game.red_hash), 6)
        self.assertEqual(len(game.blue_hash), 6)
        self.assertEqual(len(game.join_hash), 6)

        # Make sure it is red's turn.
        self.assertEqual(game.turn, False)


class JoinHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    def test_should_be_able_to_join_game(self):
        app.post('/api/create', params={'board': fixtures.SETUP})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': fixtures.SETUP,
            'join_hash': game.join_hash
        })

        game = models.Game.query().get()

        self.assertEqual(game.get_board(), [
            [
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'S'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'F'}
            ],
            [
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'}
            ],
            [
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'8'}
            ],
            [
                {u'side': 1, u'rank': u'1'},
                {u'side': 1, u'rank': u'2'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'}
            ],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [
                {u'side': 0, u'rank': u'1'},
                {u'side': 0, u'rank': u'2'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'}
            ],
            [
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'8'}
            ],
            [
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'}
            ],
            [
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'S'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'F'}
            ]
        ])


class MoveHandlerTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_move(self, pusher):
        app.post('/api/create', params={'board': fixtures.SETUP})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': fixtures.SETUP,
            'join_hash': game.join_hash
        })

        app.post('/api/move', params={
            'player_hash': game.red_hash,
            'side': 0,
            'from': json.dumps({'x': 5, 'y': 6}),
            'to': json.dumps({'x': 5, 'y': 5})
        })

        game = models.Game.query().get()

        self.assertEqual(game.get_board(), [
            [
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'S'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'F'}
            ],
            [
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'}
            ],
            [
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'8'}
            ],
            [
                {u'side': 1, u'rank': u'1'},
                {u'side': 1, u'rank': u'2'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'}
            ],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, {u'side': 0, u'rank': u'4'}, 1, 1, 0, 0],
            [
                {u'side': 0, u'rank': u'1'},
                {u'side': 0, u'rank': u'2'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'4'},
                0,
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'}
            ],
            [
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'8'}
            ],
            [
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'}
            ],
            [
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'S'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'F'}
            ]
        ])

        # Blue's turn
        self.assertEqual(game.turn, 1)

    @patch('lib.pusher.pusher.Pusher.trigger')
    def test_should_be_able_to_attack_and_draw(self, pusher):
        app.post('/api/create', params={'board': fixtures.SETUP})

        game = models.Game.query().get()

        app.post('/api/join', params={
            'board': fixtures.SETUP,
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

        self.assertEqual(game.get_board(), [
            [
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'S'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'B'},
                {u'side': 1, u'rank': u'F'}
            ],
            [
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'8'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'},
                {u'side': 1, u'rank': u'9'}
            ],
            [
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'6'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'7'},
                {u'side': 1, u'rank': u'8'}
            ],
            [
                {u'side': 1, u'rank': u'1'},
                {u'side': 1, u'rank': u'2'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'3'},
                {u'side': 1, u'rank': u'4'},
                0,
                {u'side': 1, u'rank': u'4'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'},
                {u'side': 1, u'rank': u'5'}
            ],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [
                {u'side': 0, u'rank': u'1'},
                {u'side': 0, u'rank': u'2'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'3'},
                {u'side': 0, u'rank': u'4'},
                0,
                {u'side': 0, u'rank': u'4'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'5'}
            ],
            [
                {u'side': 0, u'rank': u'5'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'6'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'7'},
                {u'side': 0, u'rank': u'8'}
            ],
            [
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'8'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'}
            ],
            [
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'9'},
                {u'side': 0, u'rank': u'S'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'B'},
                {u'side': 0, u'rank': u'F'}
            ]
        ])

        # Blue's turn
        self.assertEqual(game.turn, 1)
