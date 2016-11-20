import json
import copy


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

SETUP_0 = copy.deepcopy(SETUP)
for row in SETUP_0:
    for cell in row:
        cell['side'] = 0

SETUP_1 = copy.deepcopy(SETUP)
SETUP_1 = SETUP_1[::-1]
for i in xrange(0, len(SETUP_1)):
    SETUP_1[i] = SETUP_1[i][::-1]
for row in SETUP_1:
    for cell in row:
        cell['side'] = 1

DEFAULT_GAME = SETUP_1 + [
        [0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0, 0, 1, 1, 0, 0]
    ] + SETUP_0

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
