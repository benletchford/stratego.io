define (require) ->

  Board = require 'Board'
  Piece = require 'Piece'
  moveTypes = require 'moveTypes'

  describe 'Board', ->

    it 'should have 100 blank places', ->
      board = new Board

      expect [].concat.apply([], board._places)
        .to.have.length 100

    describe 'moving', ->

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

      it 'should allow one space move', ->
        from = x: 5, y: 5

        oneSpaceMovePermutations = [
            x: from.x + 1, y: from.y - 1
          ,
            x: from.x + 1, y: from.y
          ,
            x: from.x + 1, y: from.y + 1
          ,
            x: from.x - 1, y: from.y - 1
          ,
            x: from.x - 1, y: from.y
          ,
            x: from.x - 1, y: from.y + 1
          ,
            x: from.x, y: from.y - 1
          ,
            x: from.x, y: from.y + 1
        ]

        for to in oneSpaceMovePermutations
          board = new Board
          board.set from, @marshal

          move = board.move from, to

          expect(move).to.equal moveTypes.MOVE

      it 'should not allow more than one space move', ->
        from = x: 5, y: 5

        board = new Board
        board.set from, @marshal

        expect(->
          board.move from, x: 5, y: 7
        ).to.throw()

        expect(->
          board.move from, x: 7, y: 5
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

        for to of invalidMoves
          board = new Board
          board.set from, @marshal

          expect(->
            board.move from, to
          ).to.throw()
