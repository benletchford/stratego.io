import unittest

import models
import move_types


MOVE_TYPE_TO_NAME = [
  'move',
  'attack and draw',
  'attack and win',
  'attack and lose',
  'capture',
  'disarm',
  'assasinate'
]

MARSHAL = {
  'rank': '1',
  'side': 0
}

SCOUT = {
  'rank': '9',
  'side': 0
}

FLAG = {
  'rank': 'F',
  'side': 0
}

BOMB = {
  'rank': 'B',
  'side': 0
}


class GameTest(unittest.TestCase):
    nosegae_datastore_v3 = True

    # def setUp(self):
        # self.widget = Widget('The widget')

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

      for toPos in validMoves:
        game = models.Game()
        game.set_piece(fromPos, MARSHAL)

        move = game.check_move(fromPos, toPos)

        self.assertEqual(move, move_types.MOVE)

      for toPos in invalidMoves:
        game = models.Game()
        game.set_piece(fromPos, MARSHAL)

        self.assertRaises(Exception, game.check_move, fromPos, toPos)


    def test_should_allow_scouts_to_move_straight_in_any_direction(self):
      fromPos = {
        'x': 5,
        'y': 5
      }

      validMoves = [
          {'x': fromPos['x'] + 4, 'y': fromPos['y']}
        ,
          {'x': fromPos['x'] - 5, 'y': fromPos['y']}
        ,
          {'x': fromPos['x'], 'y': fromPos['y'] + 4}
        ,
          {'x': fromPos['x'], 'y': fromPos['y'] - 5}
      ]

      invalidMoves = [
          {'x': fromPos['x'] + 4, 'y': fromPos['y'] + 1}
        ,
          {'x': fromPos['x'] - 5, 'y': fromPos['y'] + 4}
        ,
          {'x': fromPos['x'] - 2, 'y': fromPos['y'] + 4}
        ,
          {'x': fromPos['x'] + 4, 'y': fromPos['y'] - 5}
      ]

      for toPos in validMoves:
        game = models.Game()
        game.set_piece(fromPos, SCOUT)

      for toPos in invalidMoves:
        game = models.Game()
        game.set_piece(fromPos, SCOUT)

        self.assertRaises(Exception, game.check_move, fromPos, toPos)
