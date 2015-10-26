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

    def test_should_allow_one_space_adjacent_move_not_diagonally(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        validMoves = [
            {'x': fromPos['x'] + 1, 'y': fromPos['y']},
            {'x': fromPos['x'] - 1, 'y': fromPos['y']},
            {'x': fromPos['x'], 'y': fromPos['y'] - 1},
            {'x': fromPos['x'], 'y': fromPos['y'] + 1}
        ]

        invalidMoves = [
            {'x': fromPos['x'] - 1, 'y': fromPos['y'] + 1},
            {'x': fromPos['x'] + 1, 'y': fromPos['y'] - 1},
            {'x': fromPos['x'] + 1, 'y': fromPos['y'] + 1},
            {'x': fromPos['x'] - 1, 'y': fromPos['y'] - 1},
            {'x': fromPos['x'], 'y': fromPos['y'] + 2},
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

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Illegal movement.',
                                    game.check_move,
                                    fromPos, toPos)

    def test_should_allow_scouts_to_move_straight_in_any_direction(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        validMoves = [
            {'x': fromPos['x'] + 4, 'y': fromPos['y']},
            {'x': fromPos['x'] - 5, 'y': fromPos['y']},
            {'x': fromPos['x'], 'y': fromPos['y'] + 4},
            {'x': fromPos['x'], 'y': fromPos['y'] - 5}
        ]

        invalidMoves = [
            {'x': fromPos['x'] + 4, 'y': fromPos['y'] + 1},
            {'x': fromPos['x'] - 5, 'y': fromPos['y'] + 4},
            {'x': fromPos['x'] - 2, 'y': fromPos['y'] + 4},
            {'x': fromPos['x'] + 4, 'y': fromPos['y'] - 5}
        ]

        for toPos in validMoves:
            game = models.Game()
            game.set_piece(fromPos, SCOUT)

        for toPos in invalidMoves:
            game = models.Game()
            game.set_piece(fromPos, SCOUT)

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Illegal movement.',
                                    game.check_move,
                                    fromPos, toPos)

    def test_should_not_allow_flags_to_move(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        game = models.Game()
        game.set_piece(fromPos, FLAG)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Flags cannot be moved.',
                                game.check_move,
                                fromPos, {'x': fromPos['x'], 'y': fromPos['y'] + 1})

    def test_should_not_allow_bombs_to_move(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        game = models.Game()
        game.set_piece(fromPos, BOMB)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Bombs cannot be moved.',
                                game.check_move,
                                fromPos, {'x': fromPos['x'], 'y': fromPos['y'] + 1})

    def test_should_not_allow_movement_onto_friendly_piece(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        toPos = {
            'x': fromPos['x'],
            'y': fromPos['y'] + 1
        }

        game = models.Game()
        game.set_piece(fromPos, MARSHAL)
        game.set_piece(toPos, SCOUT)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Can not move onto friendly piece.',
                                game.check_move,
                                fromPos, toPos)

    def test_should_not_allow_scouts_to_jump_over_pieces(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        toPositions = [
            {'x': 9, 'y': 5},
            {'x': 2, 'y': 5},
            {'x': 5, 'y': 8},
            {'x': 5, 'y': 2}
        ]

        inTheWayPositions = [
            {'x': 8, 'y': 5},
            {'x': 4, 'y': 5},
            {'x': 5, 'y': 6},
            {'x': 5, 'y': 3}
        ]

        notInTheWayPositions = [
            {'x': 9, 'y': 6},
            {'x': 1, 'y': 5},
            {'x': 5, 'y': 9},
            {'x': 5, 'y': 0}
        ]

        for i in xrange(len(toPositions)):
            game = models.Game()
            game.set_piece(fromPos, SCOUT)
            game.set_piece(inTheWayPositions[i], 1)

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Can not jump over pieces.',
                                    game.check_move,
                                    fromPos, toPositions[i])

        for i in xrange(len(toPositions)):
            game = models.Game()
            game.set_piece(fromPos, SCOUT)
            game.set_piece(notInTheWayPositions[i], FLAG)

            game.check_move(fromPos, toPositions[i])
