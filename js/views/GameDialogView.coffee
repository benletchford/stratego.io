define (require) ->

  template = require '../../jade/gameDialog.jade'
  attack   = require '../../jade/attack.jade'

  class extends Backbone.View
    className:  'game-dialog-view'

    initialize: (options) ->
      @$el.html template()

      @$container = @$('.game-dialog-panel-container')

      @$container.append attack()
