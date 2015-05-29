define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'
  Firebase = require 'firebase'

  BoardView = require 'BoardView'

  Pace.on 'hide', ->
    BoardView = new BoardView

    $(document.body).append BoardView.el
