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
          move = board.move from, x: 5, y: 7
        ).to.throw()

        expect(->
          move = board.move from, x: 7, y: 5
        ).to.throw()
