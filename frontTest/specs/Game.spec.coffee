define (require) ->

  Game = require '../../js/models/Game'
  moveTypes = require '../../js/moveTypes'

  Game::defaults = ->
    board: [
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ]
    turn: 0

  Game::setBlock = ({x, y}) ->
    @get('board')[y][x] = 1

  MOVE_TYPE_TO_NAME =
    0: 'move'
    1: 'attack and draw'
    2: 'attack and win'
    3: 'attack and lose'
    4: 'capture'
    5: 'disarm'
    6: 'assasinate'

  describe 'game', ->

    describe 'movement', ->

      beforeEach ->
        @marshal =
          rank: '1'
          side: 0

        @scout =
          rank: '9'
          side: 0

        @flag =
          rank: 'F'
          side: 0

        @bomb =
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
          game = new Game()
          game.setPiece from, @marshal

          move = game.checkMove from, to

          expect(move).to.equal moveTypes.MOVE

        for to in invalidMoves
          game = new Game()
          game.setPiece from, @marshal

          expect(->
            game.checkMove from, to
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
          game = new Game()
          game.setPiece from, @scout

          expect(->
            game.checkMove from, to
          ).to.not.throw()

        for to in invalidMoves
          game = new Game()
          game.setPiece from, @scout

          expect(->
            game.checkMove from, to
          ).to.throw()

      it 'should not allow flags to move', ->
        from = x: 5, y: 5

        game = new Game()
        game.setPiece from, @flag

        expect(->
          game.checkMove from, {x: from.x, y: from.y + 1}
        ).to.throw()

      it 'should not allow bombs to move', ->
        from = x: 5, y: 5

        game = new Game()
        game.setPiece from, @bomb

        expect(->
          game.checkMove from, {x: from.x, y: from.y + 1}
        ).to.throw()

      it 'should not allow movement onto friendly piece', ->
        from = x: 5, y: 5

        game = new Game()
        game.setPiece from, @marshall
        game.setPiece {x: from.x, y: from.y + 1}, @scout

        expect(->
          game.checkMove from, {x: from.x, y: from.y + 1}
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
          game = new Game()
          game.setPiece from, @scout
          game.setBlock inTheWayPositions[i]

          expect(->
            game.checkMove from, toPositions[i]
          ).to.throw()

        for i in [0...toPositions.length]
          game = new Game()
          game.setPiece from, @scout
          game.setPiece notInTheWayPositions[i], @flag

          expect(->
            game.checkMove from, toPositions[i]
          ).to.not.throw()

      it 'should not allow movement onto unmovable block', ->
        from = x: 5, y: 5

        game = new Game()
        game.setPiece from, @marshal
        game.setBlock {x: 6, y: 5}

        expect(->
          game.checkMove from, {x: 6, y: 5}
        ).to.throw()

      # All this is now done on the server end.
      describe.skip 'attacking', ->

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
                marshal =
                  rank: '1'
                  side: 0

                game = new Game()
                game.setPiece @from, marshal
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                general =
                  rank: '2'
                  side: 0

                game = new Game()
                game.setPiece @from, general
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                colonel =
                  rank: '3'
                  side: 0

                game = new Game()
                game.setPiece @from, colonel
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                major =
                  rank: '4'
                  side: 0

                game = new Game()
                game.setPiece @from, major
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                captain =
                  rank: '5'
                  side: 0

                game = new Game()
                game.setPiece @from, captain
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                lieutenant =
                  rank: '6'
                  side: 0

                game = new Game()
                game.setPiece @from, lieutenant
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                sergeant =
                  rank: '7'
                  side: 0

                game = new Game()
                game.setPiece @from, sergeant
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                miner =
                  rank: '8'
                  side: 0

                game = new Game()
                game.setPiece @from, miner
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                scout =
                  rank: '9'
                  side: 0

                game = new Game()
                game.setPiece @from, scout
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]

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
                spy =
                  rank: 'S'
                  side: 0

                game = new Game()
                game.setPiece @from, spy
                game.setPiece @to,
                  rank: toRank
                  side: 1

                expect(game.checkMove(@from, @to)).to.equal rules[toRank]
