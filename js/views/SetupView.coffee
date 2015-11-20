define (require) ->

  GameSetup = require '../models/GameSetup'
  template  = require '../../jade/setup.jade'
  ranks     = require '../ranks'

  GridView        = require './GridView'
  PanelLinkView   = require '../panel/PanelLinkView'
  PanelButtonView = require '../panel/PanelButtonView'

  class extends Backbone.View
    className: 'setup-view'

    initialize: (@options) ->
      @$el.html template()

      @$panel         = @$ '.panel'
      @$gridContainer = @$ '.grid-container'

      @$panel.append (new PanelLinkView
        title: 'Back'
        description: 'Go back to the main menu.'
        href: '#').el

      startBtn = new PanelButtonView(
        title: 'Start'
        description: 'Once you\'re happy with the setup click here to start the game.'
      )
      startBtn.$el.on 'click', =>
        Cookies.set 'lastBoard', @setup.get('board')

        @trigger 'ready',
          board: @setup.get('board')

      @$panel.append startBtn.el

      @setup = new GameSetup()

      @grid = new GridView @setup
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@swap, @)

    swap: (from, to) ->
      fromPiece = @setup.getPiece from
      toPiece   = @setup.getPiece to

      @setup.setPiece from, toPiece
      @setup.setPiece to, fromPiece

    # clickStart: ->
    #   @trigger 'move', data.from, to

      # data =
      #   board: JSON.stringify @setup.get('board')

      # switch @options.type
      #   when 'join'
      #     api_location = 'api/join'
      #     data['join_hash'] = @options.hash

      #   when 'create'
      #     api_location = 'api/create'

      #   when 'pool'
      #     api_location = 'api/pool'

      # $.post(api_location, data)
      #   .done (game) =>
      #     Cookies.set 'lastBoard', @setup.get('board')

      #     # TODO, do this better... don't use global variables.
      #     # In this case it's hard to avoid as we're using routers and we don't
      #     # want to pass information via the URL or via cookies/local storage.
      #     window._game = game
      #     window.location.hash = "play/#{game.player_hash}"

