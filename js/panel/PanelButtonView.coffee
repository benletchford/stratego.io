define (require) ->

  template = require '../../jade/panelOption.jade'

  class extends Backbone.View
    className: 'panel-button-view panel-option'
    tagName  : 'button'

    initialize: (options) ->
      options = _.defaults options,
        title      : 'Option Title'
        description: 'Description of option.'

      @$el.html template(
        title      : options.title
        description: options.description
      )
