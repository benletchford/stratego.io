define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GridView = require './GridView'
  Game     = require '../models/Game'

  template = require '../../jade/game.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (hash) ->

      $.get('api/game',
        player_hash: hash
      )
        .done (response) =>
          @$el.html template()

          @$gridContainer = @$ '.grid-container'

          @game = new Game(
            board: response.board
            turn: 0
          )

          @grid = new GridView @game
          @$gridContainer.append @grid.el
