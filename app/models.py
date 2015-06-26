import uuid
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

    def _pre_put_hook(self):
        self.red_hash = uuid.uuid4().hex[:6]
        self.blue_hash = uuid.uuid4().hex[:6]
        self.join_hash = uuid.uuid4().hex[:6]

    def getBoard(self):
        return json.loads(self.board)

    def move(self, toPos, fromPos):
        pass

    def _canMove(fromPos, toPos):
        pass

    def _isPieceBetween(fromPos, toPos, diff):
        pass

    def _attack(fromPos, toPos):
        pass
