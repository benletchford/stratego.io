define (require) ->

  Board = require 'Board'
  Piece = require 'Piece'
  moveTypes = require 'moveTypes'

  describe 'Board', ->

    it 'should have 100 blank places', ->
      board = new Board

      expect [].concat.apply([], board._places)
        .to.have.length 100

    describe 'moves', ->

      beforeEach ->
        @marshal = new Piece
          rank: '1'
          side: 0

        @scout = new Piece
          rank: '9'
          side: 0

        @flag = new Piece
          rank: 'B'
          side: 0

        @bomb = new Piece
          rank: 'B'
          side: 0

      it 'should allow one space adjacent move not diagonal', ->
        from = x: 5, y: 5

        validMoves = [
            x: from.x + 1, y: from.y
          ,
            x: from.x - 1, y: from.y
          ,
            x: from.x, y: from.y - 1
          ,
            x: from.x, y: from.y + 1
        ]

        invalidMoves = [
            x: from.x - 1, y: from.y + 1
          ,
            x: from.x + 1, y: from.y - 1
          ,
            x: from.x + 1, y: from.y + 1
          ,
            x: from.x - 1, y: from.y - 1
          ,
            x: from.x, y: from.y + 2
          ,
            x: from.x + 2, y: from.y
        ]

        for to in validMoves
          board = new Board
          board.set from, @marshal

          move = board.move from, to

          expect(move).to.equal moveTypes.MOVE

        for to in invalidMoves
          board = new Board
          board.set from, @marshal

          expect(->
            board.move from, to
          ).to.throw()

      it 'should allow scouts to move straight in any direction', ->
        from = x: 5, y: 5

        validMoves = [
            x: from.x + 4, y: from.y
          ,
            x: from.x - 5, y: from.y
          ,
            x: from.x, y: from.y + 4
          ,
            x: from.x, y: from.y - 5
        ]

        invalidMoves = [
            x: from.x + 4, y: from.y + 1
          ,
            x: from.x - 5, y: from.y + 4
          ,
            x: from.x - 2, y: from.y + 4
          ,
            x: from.x + 4, y: from.y - 5
        ]

        for to in validMoves
          board = new Board
          board.set from, @scout

          expect(->
            board.move from, to
          ).to.not.throw()

        for to in invalidMoves
          board = new Board
          board.set from, @marshal

          expect(->
            board.move from, to
          ).to.throw()
