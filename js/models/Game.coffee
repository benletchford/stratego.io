define (require) ->

  moveTypes = require '../moveTypes'

  class extends Backbone.Model

    checkMove: (from, to) ->
      fromPiece = @getPiece from
      toPiece   = @getPiece to

      unless fromPiece.side is @get('turn')
        throw new Error 'Not your turn.'

      unless fromPiece
        throw new Error 'No piece to move.'

      if toPiece
        if toPiece is 1
          throw new Error 'Can not move onto an unmoveable block.'
        if fromPiece.side is toPiece.side
          throw new Error 'Can not move onto friendly piece.'

      # Bombs and flags can't move.
      if fromPiece.rank is 'B'
        throw new Error 'Bombs cannot be moved.'
      if fromPiece.rank is 'F'
        throw new Error 'Flags cannot be moved.'

      diff = {}
      diff.x = Math.abs from.x - to.x
      diff.y = Math.abs from.y - to.y

      if diff.x is 0 and diff.y is 0
        throw new Error 'Position has not changed.'

      # We're either moving one square or we're a scout moving in a straight
      # line.
      if ((diff.x is 1) != (diff.y is 1) or (fromPiece.rank is '9')) and
         # We can't move diagonally
         (diff.x is 0) != (diff.y is 0)

        # If we're a scout we need to verify there's nothing between from and to
        if fromPiece.rank is '9' and @_isPieceBetween(to, from, diff)
          throw new Error 'Can not jump over pieces.'

        if toPiece
          return moveTypes.ATTACK

        else
          return moveTypes.MOVE

      else
        throw new Error 'Illegal movement.'

    _isPieceBetween: (from, to, diff) ->
      # We must know at this point that we're not moving on multiple axis

      # We're moving on the x axis
      if diff.y is 0
        coefficient = if from.x < to.x then 1 else -1
        for i in [1...diff.x]
          if @getPiece {x: from.x + (i * coefficient), y: from.y}
            return true

        return false

      # We're moving on the y axis
      else
        coefficient = if from.y < to.y then 1 else -1
        for i in [1...diff.y]
          if @getPiece {x: from.x, y: from.y + (i * coefficient)}
            return true

        return false

    _attack: (from, to) ->
      fromPiece = @getPiece from
      toPiece   = @getPiece to

      # Are we gonna draw?
      if fromPiece.rank is toPiece.rank
        return moveTypes.ATTACK_DRAW

      # Any movable piece can capture the flag.
      if toPiece.rank is 'F'
        return moveTypes.CAPTURE

      # Are we attacking a bomb?
      if toPiece.rank is 'B'
        if fromPiece.rank is '8'
          return moveTypes.DISARM
        else
          return moveTypes.ATTACK_LOST

      # Everything wins attacking a spy.
      if toPiece.rank is 'S'
        return moveTypes.ATTACK_WON

      # Are we a spy?
      if fromPiece.rank is 'S'
        if toPiece.rank is '1'
          return moveTypes.ASSASINATION
        else
          return moveTypes.ATTACK_LOST

      fromRank = parseInt fromPiece.rank
      toRank   = parseInt toPiece.rank

      if toRank > fromRank
        return moveTypes.ATTACK_WON
      else
        return moveTypes.ATTACK_LOST

    movePiece: (from, to) ->
      piece = @get('board')[from.y][from.x]

      @get('board')[from.y][from.x] = 0
      @get('board')[to.y][to.x]     = piece

      @trigger 'change', @

    setPiece: ({x, y}, piece) ->
      @get('board')[y][x] = piece

    getPiece: ({x, y}) ->
      @get('board')[y][x]

    flipTurn: ->
      @set 'turn', +!@get('turn')

    setLastMove: (from, to) ->
      lastMove = {from, to}
      @set('last_move', lastMove)

    setPendingAttack: (from, to) ->
      pendingAttack = {from, to}
      @set('pending_attack', pendingAttack)
