define (require) ->

  BoardView   = require './views/BoardView'
  ConsoleView = require './views/ConsoleView'
  SetupView   = require './views/SetupView'
  HomeView    = require './views/HomeView'
  GameView    = require './views/GameView'
  LoadingView = require './views/LoadingView'

  gameStates = require './gameStates'

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
      loadingView = new LoadingView text: 'Loading game...'
      @setContent loadingView.el

      $.get('api/game',
          player_hash: hash
        )
          .done (game) =>
            gameView = new GameView(game)
            @_checkGameRender gameView, loadingView

    pool: ->
      @_setup
        type: 'pool'

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

    setContent: (html) ->
      @_clear()
      @boardView.$contentContainer.html html

    _checkGameRender: (gameView, loadingView) ->
      # Loading view should already be visible when calling

      switch gameView.game.get('game_state')
        when gameStates.WAITING_FOR_OPPONENT
          loadingView.setText 'Waiting for opponent...'

          gameView.channel.bind 'blue_ready', =>
            @setContent gameView.el
            gameView.channel.unbind 'blue_ready'

        when gameStates.PLAYING
          @setContent gameView.el

    _clear: ->
      @boardView.$contentContainer.empty()

      # Remove all registered callbacks
      @stopListening()

    _setup: (setupOptions = {}, loadingText) ->
      setupView = new SetupView(setupOptions)
      @setContent setupView.el

      @listenToOnce setupView, 'ready', (data) =>
        loadingView = new LoadingView text: loadingText
        @setContent loadingView.el

        data.board = JSON.stringify data.board

        switch setupOptions.type
          when 'join'
            api_location = 'api/join'
            data['join_hash'] = setupOptions.hash

          when 'create'
            api_location = 'api/create'

          when 'pool'
            api_location = 'api/pool'

        $.post(api_location, data)
          .done (game) =>
            gameView = new GameView(game)
            @_checkGameRender gameView, loadingView

            @navigate "play/#{game.player_hash}"


