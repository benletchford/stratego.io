define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  PanelOptionView = require './PanelOptionView'

  class extends Backbone.View
    className: 'panel-view'

    initialize: ->
      option = new PanelOptionView
        title: 'Online Stratego'
        description: 'Get matched with someone online instantly.'
        href: '#join/setup'

      option2 = new PanelOptionView
        title: 'Play with a friend'
        description: 'Start a private game and invite a friend.'
        href: '#create/setup'

      @$el.append option.el
      @$el.append option2.el
