define (require) ->

    ranks = require '../ranks'

    class extends Backbone.Model

        initialize: ->
            lastBoard = Cookies.getJSON('lastBoard')

            if lastBoard
                @set('board', lastBoard)
            else
                @setDefault()

        setDefault: ->
            pieces = []
            for pieceRank, pieceDetails of ranks
                for [1..pieceDetails.amount]
                    pieces.push rank: pieceRank, side: 3

            @set('board', [])
            for i in [0..3]
                @get('board').push pieces.splice(0, 10)

        setPiece: ({x, y}, piece) ->
            @get('board')[y][x] = piece
            @trigger 'change', @

        getPiece: ({x, y}) ->
            @get('board')[y][x]

