define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../jade/setup.jade'

  class extends Backbone.View
    className: 'setup-view'

    initialize: ->
      @$el.html template()
