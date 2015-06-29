define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../../jade/grid.jade'

  class extends Backbone.View
    className: 'game-view'

    initialize: (hash) ->
      $.get('api/game',
        player_hash: hash
      )
        .done (response) ->
          console.log response
