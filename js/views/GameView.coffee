define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GridView       = require './GridView'
  GameDialogView = require './GameDialogView'
  Game           = require '../models/Game'
  moveTypes      = require '../moveTypes'

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
        @getLatest()

    render: (data) ->
      @$el.html template()

      # gameDialogView = new GameDialogView()
      # @$el.append gameDialogView.el

      @$gridContainer = @$ '.grid-container'

      @game = new Game(
        board: data.board
        turn: +data.turn
        side: data.side
        last_move: data.last_move
      )

      @grid = new GridView @game
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@move, @)

      if data.side is 0
        console.log "Join hash: #{data.join_hash}"

      # Connect to pusher...
      @pusher = new Pusher 'fd2e668a4ea4f7e23ab6', encrypted: true
      @channel = @pusher.subscribe "game-#{@hash}"

      @channel.bind 'update', (data) =>
        $.get('api/game',
          player_hash: @hash
        )
          .done _.bind @getLatest, @

    move: (from, to) ->
      # move = @game.checkMove(from, to)

      # # Flip the turn
      # @game.flipTurn()

      # @game.movePiece from, to
      # @game.setLastMove from, to

      $.post('api/move',
        player_hash: @hash
        side       : @game.get('side')
        from       : JSON.stringify from
        to         : JSON.stringify to
      )
        .done _.bind @update, @

    update: (data) ->
      if @game
        @game.set(
          board: data.board
          turn: +data.turn
          side: data.side
          last_move: data.last_move
        )
      else
        @render(data)

    getLatest: ->
      $.get('api/game',
          player_hash: @hash
        )
          .done _.bind @update, @

    # move: (from, to, local = true) ->
    #   console.log 'from: ' + JSON.stringify(from)
    #   console.log 'to: ' + JSON.stringify(to)

    #   move = @game.checkMove(from, to)

    #   # Flip the turn
    #   @game.flipTurn()

    #   switch move
    #     when moveTypes.MOVE
    #       # Clone to restore later if need be
    #       clonedGameAttri = JSON.parse(JSON.stringify(@game.attributes))

    #       @game.movePiece from, to
    #       @game.setLastMove from, to

    #       if local
    #         $.post('api/move',
    #           player_hash: @hash
    #           side       : @game.get('side')
    #           from       : JSON.stringify from
    #           to         : JSON.stringify to
    #         )
    #         .fail =>
    #           # Reset
    #           @game.set(clonedGameAttri)

    #     when moveTypes.ATTACK
    #       @game.setPendingAttack from, to


