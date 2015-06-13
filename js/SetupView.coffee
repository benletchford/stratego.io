define (require) ->

  $        = require 'jquery'
  _        = require 'underscore'
  Backbone = require 'backbone'

  template = require '../jade/setup.jade'
  piece    = require '../jade/piece.jade'

  ranks = require './ranks'

  class extends Backbone.View
    className: 'setup-view'

    events:
      'dragstart .piece': 'dragStart'

      'dragover .cell'   : 'dragOverCell'
      'dragleave .cell'  : 'dragLeaveCell'
      'drop .cell'       : 'dropCell'
      'click .cell'      : 'clickCell'
      'mouseover .cell'  : 'hoverInCell'
      'mouseleave .cell' : 'hoverOutCell'
      'dragend .piece'   : 'dragEnd'

    initialize: ->
      @$el.html template()

      @$cells = @$ '.cell'

      for pieceRank, pieceDetails of ranks
        for [1..pieceDetails.amount]
          @$cells.filter(':empty:first').html piece(rank: pieceRank, side: 3)

    dragStart: (e) ->
      $cell = $(e.target).parent()
      $cell.addClass 'selected'

      data =
        from:
          x: $cell.data 'x'
          y: $cell.data 'y'

      e.originalEvent.dataTransfer.setData 'text/plain', JSON.stringify(data)

    dragEnd: (e) ->
      $(e.target).parent().removeClass 'selected'

    dragOverCell: (e) ->
      e.preventDefault()

      $target = $(e.target)
      if $target.hasClass 'piece'
        $target = $target.parent()

      $target.addClass 'hover'

    dropCell: (e) ->
      $toCell = @_getCellFromTarget(e)
      $toCell.removeClass 'hover'

      data = JSON.parse e.originalEvent.dataTransfer.getData('text')

      to =
        x: $toCell.data 'x'
        y: $toCell.data 'y'

      console.log 'from: ' + JSON.stringify(data.from)
      console.log 'to: ' + JSON.stringify(to)

    dragLeaveCell: (e) ->
      @_getCellFromTarget(e).removeClass 'hover'

    clickCell: (e) ->
      $currentTarget = $ e.currentTarget

      $fromCell = @$cells.filter '.selected'

      @$cells.removeClass 'selected'
      @$cells.removeClass 'hover'

      if $fromCell.length
        to =
          x: $currentTarget.data 'x'
          y: $currentTarget.data 'y'

        from =
          x: $fromCell.data 'x'
          y: $fromCell.data 'y'

        console.log 'from: ' + JSON.stringify(from)
        console.log 'to: ' + JSON.stringify(to)

      # If nothing is selected us there a piece in this cell we can select?
      else if $(e.target).hasClass 'piece'
        $currentTarget.addClass 'selected'

    hoverInCell: (e) ->
      $cell = @_getCellFromTarget e

      if $cell.find('.piece').length
        $cell.addClass 'hover'
      else if @$cells.filter('.selected').length
        $cell.addClass 'hover'

    hoverOutCell: (e) ->
      @_getCellFromTarget(e).removeClass 'hover'

    _getCellFromTarget: (e) ->
      $cell = $(e.target)
      if $cell.hasClass 'piece'
        $cell = $cell.parent()

      $cell
