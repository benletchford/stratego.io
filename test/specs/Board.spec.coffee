define (require) ->

  Board = require 'Board'
  Piece = require 'Piece'
  moveTypes = require 'moveTypes'

  describe 'Board', ->

    it 'should have 100 blank places', ->
      board = new Board

      expect [].concat.apply([], board._places)
        .to.have.length 100

    describe 'movement', ->

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

      it 'should not allow flags to move', ->
        from = x: 5, y: 5

        board = new Board
        board.set from, @flag

        expect(->
          board.move from, {x: from.x, y: from.y + 1}
        ).to.throw()

      it 'should not allow bombs to move', ->
        from = x: 5, y: 5

        board = new Board
        board.set from, @bomb

        expect(->
          board.move from, {x: from.x, y: from.y + 1}
        ).to.throw()

      it 'should not allow movement onto friendly piece', ->
        from = x: 5, y: 5

        board = new Board
        board.set from, @marshall
        board.set {x: from.x, y: from.y + 1}, @scout

        expect(->
          board.move from, {x: from.x, y: from.y + 1}
        ).to.throw()

      it 'should not allow scouts to jump over pieces', ->
        from = x: 5, y: 5

        toPositions = [
            x: 9, y: 5
          ,
            x: 2, y: 5
          ,
            x: 5, y: 8
          ,
            x: 5, y: 2
        ]

        inTheWayPositions = [
            x: 8, y: 5
          ,
            x: 4, y: 5
          ,
            x: 5, y: 6
          ,
            x: 5, y: 3
        ]

        notInTheWayPositions = [
            x: 9, y: 6
          ,
            x: 1, y: 5
          ,
            x: 5, y: 9
          ,
            x: 5, y: 0
        ]

        for i in [0...toPositions.length]
          board = new Board
          board.set from, @scout
          board.setBlock inTheWayPositions[i]

          expect(->
            board.move from, toPositions[i]
          ).to.throw()

        for i in [0...toPositions.length]
          board = new Board
          board.set from, @scout
          board.set notInTheWayPositions[i], @flag

          expect(->
            board.move from, toPositions[i]
          ).to.not.throw()

      it 'should not allow movement onto unmovable block', ->
        from = x: 5, y: 5

        board = new Board
        board.set from, @marshal
        board.setBlock {x: 6, y: 5}

        expect(->
          board.move from, {x: 6, y: 5}
        ).to.throw()
