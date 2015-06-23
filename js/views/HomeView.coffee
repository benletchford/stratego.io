define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  PanelLinkView = require '../panel/PanelLinkView'

  class extends Backbone.View
    className: 'home-view panel default-panel'

    initialize: ->
      @$el.append (new PanelLinkView
        title: 'Online Stratego'
        description: 'Get matched with someone online instantly.'
        href: '#join').el

      @$el.append (new PanelLinkView
        title: 'Play with a friend'
        description: 'Start a private game with a friend.'
        href: '#create/setup').el
