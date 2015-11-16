define (require) ->

  MIN_WIDTH = 320

  class extends Backbone.View
    className:  'board-view'

    initialize: ->
      @_resize()
      $(window).on 'resize', _.debounce _.bind(@_resize, @), 100

      @$el.html '<div class="over-board-container"></div>'

      @$overboard = @$el.find '.over-board-container'

    _resize: ->
      w = $(window).width()
      h = $(window).height()

      min = Math.min w, h

      min = Math.max min, MIN_WIDTH

      @$el
        .width min
        .height min

      Backbone.trigger 'board:resize', min

    remove: ->
      $(window).off 'resize', @_resize
      Backbone.View::remove.apply this, arguments
