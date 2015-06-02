define (require) ->

  Board = require 'Board'
  Piece = require 'Piece'
  moveTypes = require 'moveTypes'

  MOVE_TYPE_TO_NAME =
    0: 'move'
    1: 'attack and draw'
    2: 'attack and win'
    3: 'attack and lose'
    4: 'capture'
    5: 'disarm'
    6: 'assasinate'

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
<<<<<<< HEAD

      describe.only 'attacking', ->

        beforeEach ->
          @from = x: 5, y: 5
          @to   = x: 5, y: 6

        describe 'marshall', ->

          rules =
            '1': moveTypes.ATTACK_DRAW
            '2': moveTypes.ATTACK_WON
            '3': moveTypes.ATTACK_WON
            '4': moveTypes.ATTACK_WON
            '5': moveTypes.ATTACK_WON
            '6': moveTypes.ATTACK_WON
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                marshal = new Piece
                  rank: '1'
                  side: 0

                board = new Board
                board.set @from, marshal
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'general', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_DRAW
            '3': moveTypes.ATTACK_WON
            '4': moveTypes.ATTACK_WON
            '5': moveTypes.ATTACK_WON
            '6': moveTypes.ATTACK_WON
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                general = new Piece
                  rank: '2'
                  side: 0

                board = new Board
                board.set @from, general
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'colonel', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_DRAW
            '4': moveTypes.ATTACK_WON
            '5': moveTypes.ATTACK_WON
            '6': moveTypes.ATTACK_WON
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                colonel = new Piece
                  rank: '3'
                  side: 0

                board = new Board
                board.set @from, colonel
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'major', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_DRAW
            '5': moveTypes.ATTACK_WON
            '6': moveTypes.ATTACK_WON
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                major = new Piece
                  rank: '4'
                  side: 0

                board = new Board
                board.set @from, major
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'captain', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_DRAW
            '6': moveTypes.ATTACK_WON
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                captain = new Piece
                  rank: '5'
                  side: 0

                board = new Board
                board.set @from, captain
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'lieutenant', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_LOST
            '6': moveTypes.ATTACK_DRAW
            '7': moveTypes.ATTACK_WON
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                lieutenant = new Piece
                  rank: '6'
                  side: 0

                board = new Board
                board.set @from, lieutenant
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'sergeant', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_LOST
            '6': moveTypes.ATTACK_LOST
            '7': moveTypes.ATTACK_DRAW
            '8': moveTypes.ATTACK_WON
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                sergeant = new Piece
                  rank: '7'
                  side: 0

                board = new Board
                board.set @from, sergeant
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'miner', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_LOST
            '6': moveTypes.ATTACK_LOST
            '7': moveTypes.ATTACK_LOST
            '8': moveTypes.ATTACK_DRAW
            '9': moveTypes.ATTACK_WON
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.DISARM
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                miner = new Piece
                  rank: '8'
                  side: 0

                board = new Board
                board.set @from, miner
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'scout', ->

          rules =
            '1': moveTypes.ATTACK_LOST
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_LOST
            '6': moveTypes.ATTACK_LOST
            '7': moveTypes.ATTACK_LOST
            '8': moveTypes.ATTACK_LOST
            '9': moveTypes.ATTACK_DRAW
            'S': moveTypes.ATTACK_WON
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                scout = new Piece
                  rank: '9'
                  side: 0

                board = new Board
                board.set @from, scout
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]

        describe 'spy', ->

          rules =
            '1': moveTypes.ASSASINATION
            '2': moveTypes.ATTACK_LOST
            '3': moveTypes.ATTACK_LOST
            '4': moveTypes.ATTACK_LOST
            '5': moveTypes.ATTACK_LOST
            '6': moveTypes.ATTACK_LOST
            '7': moveTypes.ATTACK_LOST
            '8': moveTypes.ATTACK_LOST
            '9': moveTypes.ATTACK_LOST
            'S': moveTypes.ATTACK_DRAW
            'B': moveTypes.ATTACK_LOST
            'F': moveTypes.CAPTURE

          for toRank, result of rules
            do (toRank) ->
              it "should vs #{toRank} #{MOVE_TYPE_TO_NAME[result]}", ->
                spy = new Piece
                  rank: 'S'
                  side: 0

                board = new Board
                board.set @from, spy
                board.set @to, new Piece
                  rank: toRank
                  side: 1

                expect(board.move(@from, @to)).to.equal rules[toRank]
=======
>>>>>>> [try-webpack] Webpack
