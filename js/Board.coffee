define (require) ->

  SIZE = 10

  class Board

    constructor: ->
      # Initialize a blank 10x10 matrix
      @_places = []
      for x in [0...SIZE]
        col = new Array
        col.length = SIZE

        @_places.push col

    set: ([x, y], piece) ->
      @_places[x][y] = piece

    get: ([x, y]) ->
      @_places[x][y]
