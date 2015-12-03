define (require) ->

  GridView      = require './GridView'
  Game          = require '../models/Game'
  moveTypes     = require '../moveTypes'
  pusherWrapper = require '../pusherWrapper'

  template = require '../../jade/game.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (game) ->
      @channelName = "public-game-#{game.player_hash}"

      # We should be connected to pusher at this point...
      @channel = pusherWrapper.pusher.subscribe @channelName

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
        player_hash: game.player_hash
      )

      @grid = new GridView @game
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@move, @)

      if game.side is 0
        console.log "Join hash: #{game.join_hash}"

      @channel.bind 'update', _.bind @getLatest, @

    move: (from, to) ->
      $.post('api/move',
        player_hash: @game.get('player_hash')
        side       : @game.get('side')
        from       : JSON.stringify from
        to         : JSON.stringify to
      )
        .done _.bind @setGame, @

    getLatest: ->
      $.get('api/game',
          player_hash: @game.get('player_hash')
        )
          .done _.bind @setGame, @

    setGame: (game) ->
      @game.set(
        board: game.board
        turn: +game.turn
        side: game.side
        last_move: game.last_move
        game_state: game.game_state
        player_hash: game.player_hash
      )
