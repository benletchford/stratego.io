define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../../jade/panelOption.jade'

  class extends Backbone.View
    className: 'panel-option-view'
    tagName  : 'a'

    initialize: (options) ->

      options = _.defaults options,
        href       : '#'
        title      : 'Option Title'
        description: 'Description of option.'

      @$el.attr 'href', options.href

      @$el.html template(
        title      : options.title
        description: options.description
      )
