define (require) ->

  GridView       = require './GridView'
  Game           = require '../models/Game'
  moveTypes      = require '../moveTypes'

  template = require '../../jade/game.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (game) ->
      @pusher = new Pusher 'fd2e668a4ea4f7e23ab6', encrypted: true
      @channel = @pusher.subscribe "game-#{game.player_hash}"

      @render(game)

    render: (game) ->
      @$el.html template()

      @$gridContainer = @$ '.grid-container'

      @game = new Game(
        board: game.board
        turn: +game.turn
        side: game.side
        last_move: game.last_move
        game_state: game.game_state
      )

      @grid = new GridView @game
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@move, @)

      if game.side is 0
        console.log "Join hash: #{game.join_hash}"

      @channel.bind 'update', =>
        $.get('api/game',
          player_hash: @hash
        )
          .done _.bind @getLatest, @

    move: (from, to) ->
      $.post('api/move',
        player_hash: @hash
        side       : @game.get('side')
        from       : JSON.stringify from
        to         : JSON.stringify to
      )
        .done _.bind @update, @

    update: (game) ->
      if @game
        @game.set(
          board: game.board
          turn: +game.turn
          side: game.side
          last_move: game.last_move
        )
      else
        @render(game)

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


