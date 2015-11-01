import json

from google.appengine.ext import ndb

import move_types


class InvalidMove(Exception):
    pass


class BaseModel(ndb.Model):
    created = ndb.DateTimeProperty(auto_now_add=True)
    modified = ndb.DateTimeProperty(auto_now=True)


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
        board = self.get_board()

        self.set_piece({'x': 2, 'y': 4}, 1, board)
        self.set_piece({'x': 2, 'y': 5}, 1, board)
        self.set_piece({'x': 3, 'y': 4}, 1, board)
        self.set_piece({'x': 3, 'y': 5}, 1, board)

        self.set_piece({'x': 6, 'y': 4}, 1, board)
        self.set_piece({'x': 6, 'y': 5}, 1, board)
        self.set_piece({'x': 7, 'y': 4}, 1, board)
        self.set_piece({'x': 7, 'y': 5}, 1, board)

    def get_opponent_hash(self, player_hash):
        if player_hash == self.blue_hash:
            return self.red_hash
        elif player_hash == self.red_hash:
            return self.blue_hash

    def get_board(self):
        return json.loads(self.board)

    def get_piece(self, board, pos):
        return board[pos['y']][pos['x']]

    def set_board(self, board):
        self.board = json.dumps(board)

    def set_piece(self, pos, piece, board=None):
        if board is None:
            board = self.get_board()

        board[pos['y']][pos['x']] = piece

        self.set_board(board)

    def set_last_move(self, fromPos, toPos):
        last_move = {
            'from': fromPos,
            'to': toPos
        }
        self.last_move = json.dumps(last_move)

    def move_piece(self, fromPos, toPos):
        board = self.get_board()
        piece = board[fromPos['y']][fromPos['x']]

        board[fromPos['y']][fromPos['x']] = 0

        board[toPos['y']][toPos['x']] = piece

        self.set_board(board)

        # Flip the turn
        self.turn = not self.turn

        # Set last moved
        self.set_last_move(fromPos, toPos)

        return True

    def check_move(self, fromPos, toPos):
        board = self.get_board()

        fromPiece = self.get_piece(board, fromPos)
        toPiece = self.get_piece(board, toPos)

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
            if fromPiece['rank'] == '9' and self._is_piece_between(board, fromPos, toPos, diff):
                raise InvalidMove('Can not jump over pieces.')

            if self._cell_is_occupied(toPiece):
                return self._check_attack(board, fromPiece, toPiece)

            else:
                return move_types.MOVE

        else:
            raise InvalidMove('Illegal movement.')

    def _is_piece_between(self, board, fromPos, toPos, diff):
        # We must know at this point that we're not moving on multiple axis

        # We're moving on the x axis
        if diff['y'] is 0:
            coefficient = 1 if fromPos['x'] < toPos['x'] else -1
            for i in xrange(1, diff['x']):
                if self.get_piece(board, {'x': fromPos['x'] + (i * coefficient), 'y': fromPos['y']}) != 0:
                    return True

            return False

        # We're moving on the y axis
        else:
            coefficient = 1 if fromPos['y'] < toPos['y'] else -1
            for i in xrange(1, diff['y']):
                if self.get_piece(board, {'x': fromPos['x'], 'y': fromPos['y'] + (i * coefficient)}) != 0:
                    return True

            return False

    def _check_attack(self, board, fromPiece, toPiece):

            # Are we gonna draw?
        if fromPiece['rank'] == toPiece['rank']:
            return move_types.ATTACK_DRAW

        # Any movable piece can capture the flag.
        if toPiece['rank'] == 'F':
            return move_types.CAPTURE

        # Are we attacking a bomb?
        if toPiece['rank'] == 'B':
            if fromPiece['rank'] == '8':
                return move_types.DISARM
            else:
                return move_types.ATTACK_LOST

        # Everything wins attacking a spy.
        if toPiece['rank'] == 'S':
            return move_types.ATTACK_WON

        # Are we a spy?
        if fromPiece['rank'] == 'S':
            if toPiece['rank'] == '1':
                return move_types.ASSASINATION
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
