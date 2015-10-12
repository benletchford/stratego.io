import unittest

import models


MOVE_TYPE_TO_NAME = [
  'move',
  'attack and draw',
  'attack and win',
  'attack and lose',
  'capture',
  'disarm',
  'assasinate'
]


class GameTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    def test_should_allow_one_space_adjacent_move_not_diagonally(self):
      fromPos = {
        'x': 5,
        'y': 5
      }

      validMoves = [
          {'x': fromPos['x'] + 1, 'y': fromPos['y']}
        ,
          {'x': fromPos['x'] - 1, 'y': fromPos['y']}
        ,
          {'x': fromPos['x'], 'y': fromPos['y'] - 1}
        ,
          {'x': fromPos['x'], 'y': fromPos['y'] + 1}
      ]

      invalidMoves = [
          {'x': fromPos['x'] - 1, 'y': fromPos['y'] + 1}
        ,
          {'x': fromPos['x'] + 1, 'y': fromPos['y'] - 1}
        ,
          {'x': fromPos['x'] + 1, 'y': fromPos['y'] + 1}
        ,
          {'x': fromPos['x'] - 1, 'y': fromPos['y'] - 1}
        ,
          {'x': fromPos['x'], 'y': fromPos['y'] + 2}
        ,
          {'x': fromPos['x'] + 2, 'y': fromPos['y']}
      ]

        # game = models.Game()
        # game.put()

        # self.assertEqual(6, 6)
