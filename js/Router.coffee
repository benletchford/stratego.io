define (require) ->

  Backbone = require 'backbone'

  BoardView   = require './views/BoardView'
  ConsoleView = require './views/ConsoleView'
  SetupView   = require './views/SetupView'
  HomeView    = require './views/HomeView'
  GameView    = require './views/GameView'
  LoadingView = require './views/LoadingView'

  class extends Backbone.Router
    routes:
      'setup/create'    : 'create'
      'setup/pool'      : 'pool'
      'setup/join/:hash': 'join'

      'load': 'load'

      'play/:hash': 'play'
      ''          : 'home'

    initialize: ->
      @consoleView = new ConsoleView()
      $(document.body).append @consoleView.el
      @boardView = new BoardView()
      $(document.body).append @boardView.el

    home: ->
      @boardView.$overboard.empty()

      homeView = new HomeView()
      @boardView.$overboard.html homeView.el

    play: (hash) ->
      @boardView.$overboard.empty()

      gameView = new GameView(hash)
      @boardView.$overboard.html gameView.el

    pool: ->
      @_setup
        type: 'pool'

    create: ->
      @_setup
        type: 'create'

    join: (hash) ->
      @_setup
        type: 'join'
        hash: hash

    _setup: (options = {}) ->
      @boardView.$overboard.empty()

      setupView = new SetupView(options)
      @boardView.$overboard.html setupView.el

    load: ->
      @boardView.$overboard.empty()

      loadingView = new LoadingView()
      @boardView.$overboard.html loadingView.el
