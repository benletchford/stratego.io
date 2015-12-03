import json

from google.appengine.ext import ndb

import move_types
import game_states


class InvalidMove(Exception):
    pass


class BaseModel(ndb.Model):
    created = ndb.DateTimeProperty(auto_now_add=True)
    modified = ndb.DateTimeProperty(auto_now=True)


class Pool(BaseModel):
    setup = ndb.JsonProperty(required=True)
    socket_id = ndb.StringProperty(required=True)


class Game(BaseModel):
    red_hash = ndb.StringProperty()
    blue_hash = ndb.StringProperty()
    join_hash = ndb.StringProperty()

    board = ndb.JsonProperty(default='''[
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ]''')

    red_setup = ndb.JsonProperty()
    blue_setup = ndb.JsonProperty()

    last_move = ndb.JsonProperty()

    # Who's turn is it currently? False = red, True = blue
    turn = ndb.BooleanProperty(default=False)

    # Is this game by invite only?
    private = ndb.BooleanProperty(default=True)

    game_state = ndb.IntegerProperty(default=0)

    grave_yard = ndb.JsonProperty(default='[]')

    def set_red_setup(self, red_setup):
        if not self.red_setup:
            board = self.get_board()

            board[6] = red_setup[0]
            board[7] = red_setup[1]
            board[8] = red_setup[2]
            board[9] = red_setup[3]

            self.set_board(board)
            self.red_setup = json.dumps(red_setup)
        else:
            raise AttributeError('yeah see...')

    def set_blue_setup(self, blue_setup):
        if not self.blue_setup:
            board = self.get_board()

            board[3] = blue_setup[0]
            board[2] = blue_setup[1]
            board[1] = blue_setup[2]
            board[0] = blue_setup[3]

            self.set_board(board)
            self.blue_setup = json.dumps(blue_setup)

        else:
            raise AttributeError('yeah see...')

    def set_blocks(self):
        self.set_piece({'x': 2, 'y': 4}, 1)
        self.set_piece({'x': 2, 'y': 5}, 1)
        self.set_piece({'x': 3, 'y': 4}, 1)
        self.set_piece({'x': 3, 'y': 5}, 1)

        self.set_piece({'x': 6, 'y': 4}, 1)
        self.set_piece({'x': 6, 'y': 5}, 1)
        self.set_piece({'x': 7, 'y': 4}, 1)
        self.set_piece({'x': 7, 'y': 5}, 1)

    def get_opponent_hash(self, player_hash):
        if player_hash == self.blue_hash:
            return self.red_hash
        elif player_hash == self.red_hash:
            return self.blue_hash

    def get_board(self):
        return json.loads(self.board)

    def get_piece(self, pos):
        board = self.get_board()

        return board[pos['y']][pos['x']]

    def set_board(self, new_board):
        self.board = json.dumps(new_board)

    def set_piece(self, pos, piece):
        board = self.get_board()

        board[pos['y']][pos['x']] = piece

        self.set_board(board)

    def flip_turn(self):
        self.turn = not self.turn

    def set_last_move(self, last_move):
        self.last_move = json.dumps(last_move)

    def get_last_move(self):
        if self.last_move:
            return json.loads(self.last_move)
        else:
            return {}

    def has_ended(self):
        last_move = self.get_last_move()

        if last_move and last_move['type'] == 'capture':
            return True
        else:
            return False

    def move_piece(self, fromPos, toPos):
        board = self.get_board()
        piece = board[fromPos['y']][fromPos['x']]

        board[fromPos['y']][fromPos['x']] = 0

        board[toPos['y']][toPos['x']] = piece

        self.set_board(board)

    def delete_piece(self, pos):
        board = self.get_board()
        piece = board[pos['y']][pos['x']]

        board[pos['y']][pos['x']] = 0

        self.set_board(board)

    def check_move(self, fromPos, toPos):
        fromPiece = self.get_piece(fromPos)
        toPiece = self.get_piece(toPos)

        if fromPiece == 0 or fromPiece == 1:
            raise InvalidMove('No piece to move.')

        if not fromPiece['side'] == self.turn:
            raise InvalidMove('Not your turn')

        if self._cell_is_occupied(toPiece):
            if toPiece == 1:
                raise InvalidMove('Can not move onto an unmoveable block.')
            if fromPiece['side'] == toPiece['side']:
                raise InvalidMove('Can not move onto friendly piece.')

        # Bombs and flags can't move.
        if fromPiece['rank'] == 'B':
            raise InvalidMove('Bombs cannot be moved.')
        if fromPiece['rank'] == 'F':
            raise InvalidMove('Flags cannot be moved.')

        diff = {}
        diff['x'] = abs(fromPos['x'] - toPos['x'])
        diff['y'] = abs(fromPos['y'] - toPos['y'])

        if diff['x'] == 0 and diff['y'] == 0:
            raise InvalidMove('Position has not changed.')

        # We're either moving one square or we're a scout moving in a straight
        # line.
        # We can't move diagonally
        if ((diff['x'] == 1) != (diff['y'] == 1) or (fromPiece['rank'] == '9')) and \
                (diff['x'] == 0) != (diff['y'] == 0):

            # If we're a scout we need to verify there's nothing between from
            # and to
            if fromPiece['rank'] == '9' and self._is_piece_between(fromPos, toPos, diff):
                raise InvalidMove('Can not jump over pieces.')

            if self._cell_is_occupied(toPiece):
                return self._check_attack(fromPiece, toPiece)

            else:
                return move_types.MOVE

        else:
            raise InvalidMove('Illegal movement.')

    # We must know at this point that we're not moving on multiple axis
    def _is_piece_between(self, fromPos, toPos, diff):

        board = self.get_board()

        # We're moving on the x axis
        if diff['y'] is 0:
            coefficient = 1 if fromPos['x'] < toPos['x'] else -1
            for i in xrange(1, diff['x']):
                if self.get_piece({'x': fromPos['x'] + (i * coefficient), 'y': fromPos['y']}) != 0:
                    return True

            return False

        # We're moving on the y axis
        else:
            coefficient = 1 if fromPos['y'] < toPos['y'] else -1
            for i in xrange(1, diff['y']):
                if self.get_piece({'x': fromPos['x'], 'y': fromPos['y'] + (i * coefficient)}) != 0:
                    return True

            return False

    def _check_attack(self, fromPiece, toPiece):
        # Are we gonna draw?
        if fromPiece['rank'] == toPiece['rank']:
            return move_types.ATTACK_DRAW

        # Any movable piece can capture the flag.
        if toPiece['rank'] == 'F':
            return move_types.CAPTURE

        # Are we attacking a bomb?
        if toPiece['rank'] == 'B':
            if fromPiece['rank'] == '8':
                return move_types.ATTACK_WON
            else:
                return move_types.ATTACK_LOST

        # Everything wins attacking a spy.
        if toPiece['rank'] == 'S':
            return move_types.ATTACK_WON

        # Are we a spy?
        if fromPiece['rank'] == 'S':
            if toPiece['rank'] == '1':
                return move_types.ATTACK_WON
            else:
                return move_types.ATTACK_LOST

        fromRank = int(fromPiece['rank'])
        toRank = int(toPiece['rank'])

        if toRank > fromRank:
            return move_types.ATTACK_WON
        else:
            return move_types.ATTACK_LOST

    def _cell_is_occupied(self, piece):
        return not self._cell_is_empty(piece)

    def _cell_is_empty(self, piece):
        if piece == 0:
            return True
        else:
            return False
