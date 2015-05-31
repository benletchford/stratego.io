define (require) ->

  # Plain move
  MOVE: 0

  # # Can't move to requested cell due to piece being in the way.
  # # Could be an enemy could be your own piece.
  # PIECE_IN_THE_WAY: 1

  ATTACK_DRAW: 1

  # Plain attack won
  ATTACK_WON: 2

  # Plain attack lost :(
  ATTACK_LOST: 3

  # Flag capture
  CAPTURE: 4

  # Miner removes bomb
  DISARM: 5

  # Spy kills marshall
  ASSASINATION: 6
