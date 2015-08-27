define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  GameSetup = require '../models/GameSetup'

  template = require '../../jade/setup.jade'

  ranks = require '../ranks'

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
      data =
        board: JSON.stringify @setup.get('board')

      # We're joining a game...
      if @options?.hash
        api_location = 'api/join'
        data['join_hash'] = @options.hash

      # We're creating a game...
      else
        api_location = 'api/create'

      $.post(api_location, data)
        .done (response) =>
          # TODO, do this better...
          window._response = response
          window.location.hash = "play/#{response.player_hash}"

