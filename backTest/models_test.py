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
        from_pos = {
            'x': 5,
            'y': 5
        }

        valid_moves = [
            {'x': from_pos['x'] + 1, 'y': from_pos['y']},
            {'x': from_pos['x'] - 1, 'y': from_pos['y']},
            {'x': from_pos['x'], 'y': from_pos['y'] - 1},
            {'x': from_pos['x'], 'y': from_pos['y'] + 1}
        ]

        invalid_moves = [
            {'x': from_pos['x'] - 1, 'y': from_pos['y'] + 1},
            {'x': from_pos['x'] + 1, 'y': from_pos['y'] - 1},
            {'x': from_pos['x'] + 1, 'y': from_pos['y'] + 1},
            {'x': from_pos['x'] - 1, 'y': from_pos['y'] - 1},
            {'x': from_pos['x'], 'y': from_pos['y'] + 2},
            {'x': from_pos['x'] + 2, 'y': from_pos['y']}
        ]

        for to_pos in valid_moves:
            game = models.Game()
            game.set_piece(from_pos, MARSHAL)

            move = game.check_move(from_pos, to_pos)

            self.assertEqual(move, move_types.MOVE)

        for to_pos in invalid_moves:
            game = models.Game()
            game.set_piece(from_pos, MARSHAL)

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Illegal movement.',
                                    game.check_move,
                                    from_pos, to_pos)

    def test_should_allow_scouts_to_move_straight_in_any_direction(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        valid_moves = [
            {'x': from_pos['x'] + 4, 'y': from_pos['y']},
            {'x': from_pos['x'] - 5, 'y': from_pos['y']},
            {'x': from_pos['x'], 'y': from_pos['y'] + 4},
            {'x': from_pos['x'], 'y': from_pos['y'] - 5}
        ]

        invalid_moves = [
            {'x': from_pos['x'] + 4, 'y': from_pos['y'] + 1},
            {'x': from_pos['x'] - 5, 'y': from_pos['y'] + 4},
            {'x': from_pos['x'] - 2, 'y': from_pos['y'] + 4},
            {'x': from_pos['x'] + 4, 'y': from_pos['y'] - 5}
        ]

        for to_pos in valid_moves:
            game = models.Game()
            game.set_piece(from_pos, SCOUT)

        for to_pos in invalid_moves:
            game = models.Game()
            game.set_piece(from_pos, SCOUT)

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Illegal movement.',
                                    game.check_move,
                                    from_pos, to_pos)

    def test_should_not_allow_flags_to_move(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        game = models.Game()
        game.set_piece(from_pos, FLAG)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Flags cannot be moved.',
                                game.check_move,
                                from_pos, {'x': from_pos['x'], 'y': from_pos['y'] + 1})

    def test_should_not_allow_bombs_to_move(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        game = models.Game()
        game.set_piece(from_pos, BOMB)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Bombs cannot be moved.',
                                game.check_move,
                                from_pos, {'x': from_pos['x'], 'y': from_pos['y'] + 1})

    def test_should_not_allow_movement_onto_friendly_piece(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        to_pos = {
            'x': from_pos['x'],
            'y': from_pos['y'] + 1
        }

        game = models.Game()
        game.set_piece(from_pos, MARSHAL)
        game.set_piece(to_pos, SCOUT)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Can not move onto friendly piece.',
                                game.check_move,
                                from_pos, to_pos)

    def test_should_not_allow_scouts_to_jump_over_pieces(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        to_positions = [
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

        for i in xrange(len(to_positions)):
            game = models.Game()
            game.set_piece(from_pos, SCOUT)
            game.set_piece(inTheWayPositions[i], 1)

            self.assertRaisesRegexp(models.InvalidMove,
                                    'Can not jump over pieces.',
                                    game.check_move,
                                    from_pos, to_positions[i])

        for i in xrange(len(to_positions)):
            game = models.Game()
            game.set_piece(from_pos, SCOUT)
            game.set_piece(notInTheWayPositions[i], FLAG)

            game.check_move(from_pos, to_positions[i])

    def test_should_not_allow_movement_onto_unmovable_block(self):
        from_pos = {
            'x': 5,
            'y': 5
        }

        block_pos = {'x': 6, 'y': 5}

        game = models.Game()
        game.set_piece(from_pos, MARSHAL)
        game.set_piece(block_pos, 1)

        self.assertRaisesRegexp(models.InvalidMove,
                                'Can not move onto an unmoveable block.',
                                game.check_move,
                                from_pos, block_pos)

    def test_attacking_marshall(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, MARSHAL)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_general(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, GENERAL)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_colonel(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, COLONEL)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_major(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, MAJOR)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_captain(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, CAPTAIN)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_lieutenant(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, LIEUTENANT)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_sergeant(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, SERGEANT)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_miner(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, MINER)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_scout(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, SCOUT)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)

    def test_attacking_spy(self):
        from_pos = {
            'x': 5,
            'y': 5
        }
        to_pos = {
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
            game.set_piece(from_pos, SPY)
            game.set_piece(to_pos, {'rank': key, 'side': 1})

            self.assertEqual(game.check_move(from_pos, to_pos), result)
