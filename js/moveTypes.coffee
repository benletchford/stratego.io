define (require) ->

  # Plain move
  MOVE: 0

  # # Can't move to requested cell due to piece being in the way.
  # # Could be an enemy could be your own piece.
  # PIECE_IN_THE_WAY: 1

  # Plain attack
  ATTACK: 2

  # Flag capture
  CAPTURE: 3

  # Miner removes bomb
  DISARM: 4

  # Spy kills marshall
  ASSASINATION: 5
