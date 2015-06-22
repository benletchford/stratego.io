define (require) ->

  Backbone = require 'backbone'

  BoardView = require './views/BoardView'
  SetupView = require './views/SetupView'
  HomeView = require './views/HomeView'

  class extends Backbone.Router
    routes:
      'create' : 'create'
      ''             : 'home'

    initialize: ->
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
