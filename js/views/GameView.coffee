define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GridView   = require './GridView'
  AttackView = require './AttackView'
  Game       = require '../models/Game'

  template = require '../../jade/game.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (@hash) ->
      # TODO, do this better... no need to send ajax request when we already
      # know the data from setup.
      if window._response
        @render(window._response)
        delete window._response

      else
        $.get('api/game',
          player_hash: @hash
        )
          .done _.bind @render, @

    render: (data) ->
      @$el.html template()

      # attackView = new AttackView()
      # @$el.append attackView.el

      @$gridContainer = @$ '.grid-container'

      @game = new Game(
        board: data.board
        turn: +data.turn
        side: data.side
      )

      @grid = new GridView @game
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@move, @)

      if data.side is 0
        console.log "Join hash: #{data.join_hash}"

      # Connect to pusher...
      @pusher = new Pusher 'fd2e668a4ea4f7e23ab6', encrypted: true
      @channel = @pusher.subscribe "game-#{@hash}"

      @channel.bind 'move', (data) =>
        @move(data.from, data.to, false)

    move: (from, to, local = true) ->
      console.log 'from: ' + JSON.stringify(from)
      console.log 'to: ' + JSON.stringify(to)

      move = @game.canMove(from, to)

      # Flip the turn
      @game.flipTurn()

      if move is 0
        @game.movePiece from, to

        if local
          $.post('api/move',
            player_hash: @hash
            side       : @game.get('side')
            from       : JSON.stringify from
            to         : JSON.stringify to
          )

      # else if move is 1
      #   c
