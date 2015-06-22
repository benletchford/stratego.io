define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  PanelOptionView = require '../panel/PanelOptionView'

  class extends Backbone.View
    className: 'home-view panel default-panel'

    initialize: ->
      @$el.append (new PanelOptionView
        title: 'Online Stratego'
        description: 'Get matched with someone online instantly.'
        href: '#join').el

      @$el.append (new PanelOptionView
        title: 'Play with a friend'
        description: 'Start a private game with a friend.'
        href: '#create').el
