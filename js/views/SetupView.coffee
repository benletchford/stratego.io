define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../../jade/setup.jade'
  piece    = require '../../jade/piece.jade'

  ranks = require '../ranks'

  GridController = require '../controllers/GridController'

  class extends Backbone.View
    className: 'setup-view'

    initialize: ->
      @$el.html template()

      @$grid  = @$ '.setup-grid'
      @$cells = @$ '.cell'

      for pieceRank, pieceDetails of ranks
        for [1..pieceDetails.amount]
          @$cells.filter(':empty:first').html piece(rank: pieceRank, side: 3)

      @$gridController = new GridController @$grid

      @$gridController.move = _.bind @swap, @

    cordinatesToCell: (co) ->
      @$cells.filter("[data-x=#{co.x}][data-y=#{co.y}]")

    swap: (from, to) ->
      $from = @cordinatesToCell from
      $to   = @cordinatesToCell to

      $fromPiece = $from.find '.piece'
      $toPiece   = $to.find '.piece'

      $toPiece.appendTo $from
      $fromPiece.appendTo $to