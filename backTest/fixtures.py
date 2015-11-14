import json


SETUP = [
    [
        {'rank': '1', 'side': 3},
        {'rank': '2', 'side': 3},
        {'rank': '3', 'side': 3},
        {'rank': '3', 'side': 3},
        {'rank': '4', 'side': 3},
        {'rank': '4', 'side': 3},
        {'rank': '4', 'side': 3},
        {'rank': '5', 'side': 3},
        {'rank': '5', 'side': 3},
        {'rank': '5', 'side': 3}
    ],
    [
        {'rank': '5', 'side': 3},
        {'rank': '6', 'side': 3},
        {'rank': '6', 'side': 3},
        {'rank': '6', 'side': 3},
        {'rank': '6', 'side': 3},
        {'rank': '7', 'side': 3},
        {'rank': '7', 'side': 3},
        {'rank': '7', 'side': 3},
        {'rank': '7', 'side': 3},
        {'rank': '8', 'side': 3}
    ],
    [
        {'rank': '8', 'side': 3},
        {'rank': '8', 'side': 3},
        {'rank': '8', 'side': 3},
        {'rank': '8', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3}
    ],
    [
        {'rank': '9', 'side': 3},
        {'rank': '9', 'side': 3},
        {'rank': 'S', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'B', 'side': 3},
        {'rank': 'F', 'side': 3}
    ]
]

DEFAULT_GAME = [
    [
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': 'S', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'B', 'side': 1},
        {'rank': 'F', 'side': 1}
    ],
    [
        {'rank': '8', 'side': 1},
        {'rank': '8', 'side': 1},
        {'rank': '8', 'side': 1},
        {'rank': '8', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1},
        {'rank': '9', 'side': 1}
    ],
    [
        {'rank': '5', 'side': 1},
        {'rank': '6', 'side': 1},
        {'rank': '6', 'side': 1},
        {'rank': '6', 'side': 1},
        {'rank': '6', 'side': 1},
        {'rank': '7', 'side': 1},
        {'rank': '7', 'side': 1},
        {'rank': '7', 'side': 1},
        {'rank': '7', 'side': 1},
        {'rank': '8', 'side': 1}
    ],
    [
        {'rank': '1', 'side': 1},
        {'rank': '2', 'side': 1},
        {'rank': '3', 'side': 1},
        {'rank': '3', 'side': 1},
        {'rank': '4', 'side': 1},
        {'rank': '4', 'side': 1},
        {'rank': '4', 'side': 1},
        {'rank': '5', 'side': 1},
        {'rank': '5', 'side': 1},
        {'rank': '5', 'side': 1}
    ],
    [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
    [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
    [
        {'rank': '1', 'side': 0},
        {'rank': '2', 'side': 0},
        {'rank': '3', 'side': 0},
        {'rank': '3', 'side': 0},
        {'rank': '4', 'side': 0},
        {'rank': '4', 'side': 0},
        {'rank': '4', 'side': 0},
        {'rank': '5', 'side': 0},
        {'rank': '5', 'side': 0},
        {'rank': '5', 'side': 0}
    ],
    [
        {'rank': '5', 'side': 0},
        {'rank': '6', 'side': 0},
        {'rank': '6', 'side': 0},
        {'rank': '6', 'side': 0},
        {'rank': '6', 'side': 0},
        {'rank': '7', 'side': 0},
        {'rank': '7', 'side': 0},
        {'rank': '7', 'side': 0},
        {'rank': '7', 'side': 0},
        {'rank': '8', 'side': 0}
    ],
    [
        {'rank': '8', 'side': 0},
        {'rank': '8', 'side': 0},
        {'rank': '8', 'side': 0},
        {'rank': '8', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0}
    ],
    [
        {'rank': '9', 'side': 0},
        {'rank': '9', 'side': 0},
        {'rank': 'S', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'B', 'side': 0},
        {'rank': 'F', 'side': 0}
    ]
]

MARSHAL = {
    'rank': '1',
    'side': 0
}

GENERAL = {
    'rank': '2',
    'side': 0
}

COLONEL = {
    'rank': '3',
    'side': 0
}

MAJOR = {
    'rank': '4',
    'side': 0
}

CAPTAIN = {
    'rank': '5',
    'side': 0
}

LIEUTENANT = {
    'rank': '6',
    'side': 0
}

SERGEANT = {
    'rank': '7',
    'side': 0
}

MINER = {
    'rank': '8',
    'side': 0
}

SCOUT = {
    'rank': '9',
    'side': 0
}

SPY = {
    'rank': 'S',
    'side': 0
}

FLAG = {
    'rank': 'F',
    'side': 0
}

BOMB = {
    'rank': 'B',
    'side': 0
}
