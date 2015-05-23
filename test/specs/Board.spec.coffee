define (require) ->

  Board = require 'Board'
  Piece = require 'Piece'

  describe 'Board', ->

    it 'should have 100 blank places', ->
      board = new Board

      expect [].concat.apply([], board._places)
        .to.have.length 100
