define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  BoardView = require 'BoardView'

  Pace.on 'hide', ->
    BoardView = new BoardView

    $(document.body).append BoardView.el
