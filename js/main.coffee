define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'
  Pace     = require 'pace'

  BoardView = require 'BoardView'

  Pace.on 'done', ->
    BoardView = new BoardView

    $(document.body).append BoardView.el
