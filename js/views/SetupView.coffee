define (require) ->

  GameSetup    = require '../models/GameSetup'
  template     = require '../../jade/setup.jade'
  setupTextbox = require '../../jade/setupTextbox.jade'
  ranks        = require '../ranks'

  GridView         = require './GridView'
  PanelLinkView    = require '../panel/PanelLinkView'
  PanelButtonView  = require '../panel/PanelButtonView'
  PanelTextboxView = require '../panel/PanelTextboxView'

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

      @$panel.append (new PanelTextboxView
        html: setupTextbox()
      ).el

      @setup = new GameSetup()

      @grid = new GridView @setup
      @$gridContainer.append @grid.el

      @listenTo @grid, 'move', _.bind(@swap, @)

    swap: (from, to) ->
      fromPiece = @setup.getPiece from
      toPiece   = @setup.getPiece to

      @setup.setPiece from, toPiece
      @setup.setPiece to, fromPiece
