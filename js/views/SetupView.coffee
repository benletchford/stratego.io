define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GameSetup = require '../models/GameSetup.coffee'

  template = require '../../jade/setup.jade'
  piece    = require '../../jade/piece.jade'

  ranks = require '../ranks'

  GridView        = require './GridView'
  PanelLinkView   = require '../panel/PanelLinkView'
  PanelButtonView = require '../panel/PanelButtonView'

  class extends Backbone.View
    className: 'setup-view'

    initialize: ->
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
      startBtn.$el.on 'click', _.bind @clickStart, @

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

    clickStart: ->
      $.post 'api/create',
        board: JSON.stringify @setup.get('board')
