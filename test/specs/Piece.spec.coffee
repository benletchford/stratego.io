define (require) ->

  Piece = require 'Piece'
  ranks = require 'ranks'

  describe 'Piece', ->

    it 'should not throw if rank is valid', ->
      for rank of ranks
        expect(->
          new Piece
            rank: rank
            side: 0
        ).to.not.throw()

    it 'should throw if rank is not valid', ->
      expect(->
        new Piece
          rank: 'abc'
          side: 0
      ).to.throw()

    it 'should not throw if side 0 or 1', ->
      expect(->
        new Piece
          rank: '1'
          side: 0
      ).to.not.throw()

      expect(->
        new Piece
          rank: '1'
          side: 1
      ).to.not.throw()

    it 'should throw if side not valid', ->
      expect(->
        new Piece
          rank: '1'
          side: 2
      ).to.throw()

      expect(->
        new Piece
          rank: '1'
          side: -1
      ).to.throw()
