define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  Setup = require '../models/Setup.coffee'

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


      window.setup = new Setup()
      @grid = new GridView window.setup 
      @$gridContainer.append @grid.el

      # @grid.listenTo 'move', _.bind @swap, @

    cordinatesToCell: (co) ->
      @$cells.filter("[data-x=#{co.x}][data-y=#{co.y}]")

    swap: (from, to) ->
      $from = @cordinatesToCell from
      $to   = @cordinatesToCell to

      $fromPiece = $from.find '.piece'
      $toPiece   = $to.find '.piece'

      $toPiece.appendTo $from
      $fromPiece.appendTo $to

    clickStart: ->

