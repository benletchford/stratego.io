define (require) ->

  moveTypes = require 'moveTypes'

  SIZE = 10

  class Board

    constructor: ->
      # Initialize a blank 10x10 matrix.
      @_places = []
      for x in [0...SIZE]
        col = new Array
        col.length = SIZE

        @_places.push col

    set: (position, piece) ->
      @_places[position.x][position.y] = piece

    get: (position) ->
      @_places[position.x][position.y]

    move: (from, to) ->
      fromPiece = @get from
      toPiece   = @get to

      unless fromPiece
        throw new Error 'Nothing to move.'

      # Bombs and flags can't move.
      if fromPiece.rank is 'B'
        throw new Error 'Bombs cannot be moved.'
      if fromPiece.rank is 'F'
        throw new Error 'Flags cannot be moved.'

      diff = {}
      diff.x = Math.abs from.x - to.x
      diff.y = Math.abs from.y - to.y

      # Position hasn't changed.
      if diff.x is 0 and diff.y is 0
        throw new Error 'Invalid move.'

      # We're either moving one square or we're a scout moving in a straight
      # line.
      if ((diff.x is 1) != (diff.y is 1) or (fromPiece.rank is '9')) and
         # We can't move diagonally
         (diff.x is 0) != (diff.y is 0)

        return moveTypes.MOVE

      else
        throw new Error 'Invalid move.'
