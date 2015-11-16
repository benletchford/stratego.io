define (require) ->

  template = require '../../jade/attack.jade'

  class extends Backbone.View
    className:  'attack-view'

    initialize: ->
      @$el.html template()
