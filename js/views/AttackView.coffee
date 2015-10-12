define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../../jade/attack.jade'

  class extends Backbone.View
    className:  'attack-view'

    initialize: ->
      @$el.html template()
