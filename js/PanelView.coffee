define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  class extends Backbone.View
    className:  'panel-view'

    initialize: ->
      @$el.text 'something like this.'
