define (require) ->

  template = require '../../jade/loading.jade'

  class extends Backbone.View
    className:  'loading-view'

    initialize: (@options) ->
      @$el.html template()

      @$spinnerContainer = @$ '.spinner-container'
      @$loadingText      = @$ '.loading-text'

      @spinner = new Spinner().spin()
      @$spinnerContainer.html @spinner.el

      _.defaults @options,
        text: 'Loading...'

      @setText @options.text

    setText: (text) ->
      @$loadingText.text text

    stop: ->
      @spinner.stop()
