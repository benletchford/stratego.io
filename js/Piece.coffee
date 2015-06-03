define (require) ->

  ranks = require './ranks'

  class Piece

    constructor: ({@rank, @side}) ->

      unless @rank of ranks
        throw new Error 'Invalid rank.'

      unless @side in [0..1]
        throw new Error 'Side should be 0 or 1.'
