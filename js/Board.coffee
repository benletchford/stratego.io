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
        throw new Error "Bombs can't be moved."
      if fromPiece.rank is 'F'
        throw new Error "Flags can't be moved."

      diff = {}
      diff.x = from.x - to.x
      diff.y = from.y - to.y

      # Position hasn't changed.
      if diff.x is 0 and diff.y is 0
        throw new Error 'Invalid move.'

      # We're moving one square.
      if (Math.abs(diff.x) in [0..1] and Math.abs(diff.y) in [0..1]) or
         # Scouts can move in straight lines.
         (fromPiece.rank is '9' and ((diff.x isnt 0) != (diff.y isnt 0)))

        return moveTypes.MOVE

      else
        throw new Error 'Invalid move.'
