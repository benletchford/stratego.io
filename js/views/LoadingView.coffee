define (require) ->

  template = require '../../jade/loading.jade'

  class extends Backbone.View
    className:  'loading-view'

    initialize: ->
      @$el.html template()

      @$spinnerContainer = @$ '.spinner-container'
      @$loadingText      = @$ '.loading-text'

      spinner = new Spinner().spin()
      @$spinnerContainer.html spinner.el

      @setLoadingText('Loading...')

    setLoadingText: (text) ->
      @$loadingText.text(text)
