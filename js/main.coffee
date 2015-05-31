define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  BoardView = require 'BoardView'
  PanelView = require 'PanelView'
  Board     = require 'Board'
  Piece     = require 'Piece'

  done = ->
    BoardView = new BoardView
    PanelView = new PanelView

    $(document.body).append BoardView.el

    BoardView.$el.append PanelView.el

  # Due to a potential race condition Pace could finish before the hide event
  # is bound and it will never be triggered :(
  if Pace.running is false
    done()
  else
    Pace.on 'hide', done
