define (require) ->

  class extends Backbone.View
    tagName  : 'img'
    className: 'console-view'

    initialize: ->
      @listenTo Backbone, 'board:resize', @resize

    resize: (boardWidth) ->
        w = $(window).width() - boardWidth
        h = $(window).height()

        if w > 199
          @$el.removeClass 'hidden'
          @$el.attr 'src', ''

          @$el.width w
          @$el.height h

          @$el.attr 'src', @getImageURL(w, h)

        else
          @$el.addClass 'hidden'

    getImageURL: (w, h) ->
      "http://lorempixel.com/g/#{w}/#{h}/animals"
