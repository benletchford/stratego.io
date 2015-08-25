define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GridView = require './GridView'
  Game     = require '../models/Game'

  template = require '../../jade/game.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (@hash) ->
      $.get('api/game',
        player_hash: @hash
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

          @listenTo @grid, 'move', _.bind(@move, @)

          @connect()

    move: (from, to) ->
      console.log 'from: ' + JSON.stringify(from)
      console.log 'to: ' + JSON.stringify(to)

      move = @game.canMove(from, to)

      if move is 0
        @game.movePiece from, to
        console.log 'yes'

    connect: ->
      @pusher = new Pusher 'fd2e668a4ea4f7e23ab6', encrypted: true

      @channel = @pusher.subscribe(@hash)

      @channel.bind 'opponent_move', (data) ->
        console.log data.message

