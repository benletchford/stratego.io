import json

from lib import ndb_json


def get_sendable_game(game, side):
    game_dict = json.loads(ndb_json.dumps(game))

    if side == 0:
        game_dict['player_hash'] = game_dict['red_hash']

    elif side == 1:
        game_dict['player_hash'] = game_dict['blue_hash']

    # These are secret and should never be sent.
    del game_dict['red_hash']
    del game_dict['blue_hash']
    del game_dict['red_setup']
    del game_dict['blue_setup']

    game_dict['side'] = side

    if game_dict['last_move']:
        game_dict['last_move'] = json.loads(game_dict['last_move'])
    else:
        del game_dict['last_move']

    game_dict['board'] = get_sendable_board(game, side)

    return game_dict


def get_sendable_board(game, side):
    board = game.get_board()

    # Only continue if the game hasn't finished
    if game.has_ended():
        return board

    if side == 0 and not game.blue_setup:
        board[0] = unknown_row(side)
        board[1] = unknown_row(side)
        board[2] = unknown_row(side)
        board[3] = unknown_row(side)

        return board

    else:
        return hide_side(board, side)


def hide_side(board, side):
    for y in xrange(10):
        for x in xrange(10):
            if is_cell_occupied(board[y][x]) and board[y][x]['side'] != side:
                board[y][x] = unknown(side)

    return board


def is_cell_occupied(cell):
    if cell == 0 or cell == 1:
        return False
    else:
        return True


def unknown(side):
    return {'rank': '?', 'side': OPPOSITE_SIDE[side]}


def unknown_row(side):
    return [unknown(side), unknown(side), unknown(side), unknown(side),
            unknown(side), unknown(side), unknown(side), unknown(side), unknown(side), unknown(side)]


OPPOSITE_SIDE = [1, 0]
