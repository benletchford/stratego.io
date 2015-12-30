define (require) ->

  class extends Backbone.View
    className: 'panel-textbox-view panel-textbox'

    initialize: (options) ->
      options = _.defaults options,
        html: 'Text stuff!'

      @$el.html options.html
