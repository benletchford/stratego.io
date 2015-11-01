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

GENERAL = {
    'rank': '2',
    'side': 0
}

COLONEL = {
    'rank': '3',
    'side': 0
}

MAJOR = {
    'rank': '4',
    'side': 0
}

CAPTAIN = {
    'rank': '5',
    'side': 0
}

LIEUTENANT = {
    'rank': '6',
    'side': 0
}

SERGEANT = {
    'rank': '7',
    'side': 0
}

MINER = {
    'rank': '8',
    'side': 0
}

SCOUT = {
    'rank': '9',
    'side': 0
}

SPY = {
    'rank': 'S',
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

    def test_should_not_allow_movement_onto_unmovable_block(self):
        fromPos = {
            'x': 5,
            'y': 5
        }

        block_pos = {'x': 6, 'y': 5}

        game = models.Game()
        game.set_piece(fromPos, MARSHAL)
        game.set_piece(block_pos, 1)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Can not move onto an unmoveable block.',
                                game.check_move,
                                fromPos, block_pos)

    def test_attacking_marshall(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_DRAW,
            '2': move_types.ATTACK_WON,
            '3': move_types.ATTACK_WON,
            '4': move_types.ATTACK_WON,
            '5': move_types.ATTACK_WON,
            '6': move_types.ATTACK_WON,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, MARSHAL)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_general(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_DRAW,
            '3': move_types.ATTACK_WON,
            '4': move_types.ATTACK_WON,
            '5': move_types.ATTACK_WON,
            '6': move_types.ATTACK_WON,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, GENERAL)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_colonel(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_DRAW,
            '4': move_types.ATTACK_WON,
            '5': move_types.ATTACK_WON,
            '6': move_types.ATTACK_WON,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, COLONEL)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_major(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_DRAW,
            '5': move_types.ATTACK_WON,
            '6': move_types.ATTACK_WON,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, MAJOR)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_captain(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_DRAW,
            '6': move_types.ATTACK_WON,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, CAPTAIN)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_lieutenant(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_LOST,
            '6': move_types.ATTACK_DRAW,
            '7': move_types.ATTACK_WON,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, LIEUTENANT)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_sergeant(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_LOST,
            '6': move_types.ATTACK_LOST,
            '7': move_types.ATTACK_DRAW,
            '8': move_types.ATTACK_WON,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, SERGEANT)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_miner(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_LOST,
            '6': move_types.ATTACK_LOST,
            '7': move_types.ATTACK_LOST,
            '8': move_types.ATTACK_DRAW,
            '9': move_types.ATTACK_WON,
            'S': move_types.ATTACK_WON,
            'B': move_types.DISARM,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, MINER)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_scout(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ATTACK_LOST,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_LOST,
            '6': move_types.ATTACK_LOST,
            '7': move_types.ATTACK_LOST,
            '8': move_types.ATTACK_LOST,
            '9': move_types.ATTACK_DRAW,
            'S': move_types.ATTACK_WON,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, SCOUT)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)

    def test_attacking_spy(self):
        fromPos = {
            'x': 5,
            'y': 5
        }
        toPos = {
            'x': 5,
            'y': 6
        }

        rules = {
            '1': move_types.ASSASINATION,
            '2': move_types.ATTACK_LOST,
            '3': move_types.ATTACK_LOST,
            '4': move_types.ATTACK_LOST,
            '5': move_types.ATTACK_LOST,
            '6': move_types.ATTACK_LOST,
            '7': move_types.ATTACK_LOST,
            '8': move_types.ATTACK_LOST,
            '9': move_types.ATTACK_LOST,
            'S': move_types.ATTACK_DRAW,
            'B': move_types.ATTACK_LOST,
            'F': move_types.CAPTURE
        }

        for key, result in rules.iteritems():
            game = models.Game()
            game.set_piece(fromPos, SPY)
            game.set_piece(toPos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(fromPos, toPos), result)
