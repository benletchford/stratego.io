define (require) ->

  Backbone = require 'backbone'

  BoardView   = require './views/BoardView'
  ConsoleView = require './views/ConsoleView'
  SetupView   = require './views/SetupView'
  HomeView    = require './views/HomeView'
  GameView    = require './views/GameView'

  class extends Backbone.Router
    routes:
      'create/setup': 'create'
      'play/:hash'  : 'play'
      ''            : 'home'

    initialize: ->
      @consoleView = new ConsoleView()
      $(document.body).append @consoleView.el
      @boardView = new BoardView()
      $(document.body).append @boardView.el

    home: ->
      @boardView.$overboard.empty()

      homeView = new HomeView()
      @boardView.$overboard.html homeView.el

    create: ->
      @boardView.$overboard.empty()

      setupView = new SetupView()
      @boardView.$overboard.html setupView.el

    play: (hash) ->
      @boardView.$overboard.empty()

      gameView = new GameView(hash)
      @boardView.$overboard.html gameView.el
