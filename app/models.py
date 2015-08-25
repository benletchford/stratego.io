import json

from google.appengine.ext import ndb


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
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ]''')

    red_setup = ndb.JsonProperty()
    blue_setup = ndb.JsonProperty()

    # Who's turn is it currently? False = red, True = blue
    turn = ndb.BooleanProperty(default=False)

    # Is this game by invite only?
    private = ndb.BooleanProperty(default=True)

    # def _pre_put_hook(self):
    #     self.red_hash = uuid.uuid4().hex[:6]
    #     self.blue_hash = uuid.uuid4().hex[:6]
    #     self.join_hash = uuid.uuid4().hex[:6]

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

    def get_board(self):
        return json.loads(self.board)

    def set_board(self, board):
        self.board = json.dumps(board)

    def move(self, fromPos, toPos):
        board = self.get_board()
        piece = board[fromPos['y']][fromPos['x']]

        board[fromPos['y']][fromPos['x']] = 0

        board[toPos['y']][toPos['x']] = piece

        self.set_board(board)

    def _canMove(fromPos, toPos):
        pass

    def _isPieceBetween(fromPos, toPos, diff):
        pass

    def _attack(fromPos, toPos):
        pass
