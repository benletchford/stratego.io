import uuid

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

    # Who's turn is it currently? False = red, True = blue
    turn = ndb.BooleanProperty(default=False)

    # Is this game by invite only?
    private = ndb.BooleanProperty(default=True)

    def _pre_put_hook(self):
        self.red_hash = uuid.uuid4().hex
        self.blue_hash = uuid.uuid4().hex
        self.join_hash = uuid.uuid4().hex

    def move(self, to, from):
        pass

    def _canMove(from, to):
        pass

    def _isPieceBetween(from, to, diff):
        pass

    def _attack(from, to):
        pass
