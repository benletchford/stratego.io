define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  class extends Backbone.View
    className:  'board-view'

    initialize: ->
      @_resize()
      $(window).on 'resize', _.debounce _.bind(@_resize, @), 100

    _resize: ->
      w = $(window).width()
      h = $(window).height()

      min = Math.min w, h

      @$el
        .width min
        .height min

    remove: ->
      $(window).off 'resize', @_resize
      Backbone.View::remove.apply this, arguments
