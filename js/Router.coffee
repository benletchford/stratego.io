define (require) ->

  BoardView   = require './views/BoardView'
  ConsoleView = require './views/ConsoleView'
  SetupView   = require './views/SetupView'
  HomeView    = require './views/HomeView'
  GameView    = require './views/GameView'
  LoadingView = require './views/LoadingView'

  gameStates    = require './gameStates'
  pusherWrapper = require './pusherWrapper'

  class extends Backbone.Router
    routes:
      'play/:hash'      : 'play'
      'setup/create'    : 'create'
      'setup/join/:hash': 'join'
      'setup/pool'      : 'pool'
      ''                : 'home'

    initialize: ->
      @consoleView = new ConsoleView()
      $(document.body).append @consoleView.el
      @boardView = new BoardView()
      $(document.body).append @boardView.el

    home: ->
      homeView = new HomeView()
      @setContent homeView.el

    play: (hash) ->
      loadingView = new LoadingView html: 'Loading game...'
      @setContent loadingView.el

      $.get('api/game',
          player_hash: hash
        )
          .done (game) =>
            @_checkGameRender game, loadingView

    pool: ->
      @_setup
        type: 'pool'
      ,
        'Connecting to pool...'

    create: ->
      @_setup
        type: 'create'
      ,
        'Creating game...'

    join: (hash) ->
      @_setup
        type: 'join'
        hash: hash
      ,
        'Joining game...'

    setContent: (html, exceptedChannelNames = []) ->
      @_clear(exceptedChannelNames)
      @boardView.$contentContainer.html html

    _checkGameRender: (game, loadingView) ->
      # Loading view should already be visible when calling

      loadingView.setHtml 'Connecting to websocket...'
      pusherWrapper.connect()
        .done =>
          gameView = new GameView(game)

          switch gameView.game.get('game_state')
            when gameStates.WAITING_FOR_OPPONENT
              joinUrl = location.protocol + '//' + location.host \
                + "#setup/join/#{game.join_hash}"

              loadingView.setHtml(
                "Waiting for opponent...<br /><br /> #{joinUrl}"
              )

              gameView.channel.bind 'blue_ready', =>
                @setContent gameView.el, [gameView.channelName]
                gameView.channel.unbind 'blue_ready'

            when gameStates.PLAYING
              @setContent gameView.el, [gameView.channelName]

    _clear: (exceptedChannelNames)->
      @boardView.$contentContainer.empty()

      # Remove all registered callbacks
      @stopListening()

      # Unsubscribe from all channels (except those passed) and unbind all
      # events...
      pusherWrapper.unsubscribeAll exceptedChannelNames

    _joinPool: (board, loadingView) ->
      # Loading view should already be visible when calling

      pusherWrapper.connect()
        .done =>
          loadingView.setHtml 'Connected to pool, setting up match...'

          socketId = pusherWrapper.pusher.connection.socket_id

          $.post('api/pool/join',
            board: board
            socket_id: socketId
          )
            .done (game) =>
              loadingView.setHtml 'In pool, waiting for an opponent...'

              channel = pusherWrapper.pusher.subscribe("public-pool-#{socketId}")
              channel.bind 'opponent_found', (data) =>
                @navigate "play/#{data.player_hash}", trigger: true

    _setup: (setupOptions = {}, loadingHtml) ->
      setupView = new SetupView(setupOptions)
      @setContent setupView.el

      @listenToOnce setupView, 'ready', (data) =>
        loadingView = new LoadingView html: loadingHtml
        @setContent loadingView.el

        data.board = JSON.stringify data.board
        if setupOptions.type is 'join'
          data.join_hash = setupOptions.hash

        switch setupOptions.type
          when 'create', 'join'
            $.post("api/#{setupOptions.type}", data)
              .done (game) =>
                @_checkGameRender game, loadingView

                @navigate "play/#{game.player_hash}"

          when 'pool'
            @_joinPool data.board, loadingView
