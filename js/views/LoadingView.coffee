define (require) ->

  template = require '../../jade/loading.jade'

  class extends Backbone.View
    className:  'loading-view'

    initialize: (@options) ->
      @$el.html template()

      @$spinnerContainer = @$ '.spinner-container'
      @$loadingHtml      = @$ '.loading-html'

      @spinner = new Spinner().spin()
      @$spinnerContainer.html @spinner.el

      _.defaults @options,
        html: 'Loading...'

      @setHtml @options.html

    setHtml: (html) ->
      @$loadingHtml.html html

    stop: ->
      @spinner.stop()
