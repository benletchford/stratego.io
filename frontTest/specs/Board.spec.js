(function() {
  define(function(require) {
    var Board, MOVE_TYPE_TO_NAME, Piece, moveTypes;
    Board = require('Board');
    Piece = require('Piece');
    moveTypes = require('moveTypes');
    MOVE_TYPE_TO_NAME = {
      0: 'move',
      1: 'attack and draw',
      2: 'attack and win',
      3: 'attack and lose',
      4: 'capture',
      5: 'disarm',
      6: 'assasinate'
    };
    return describe('Board', function() {
      it('should have 100 blank places', function() {
        var board;
        board = new Board;
        return expect([].concat.apply([], board._places)).to.have.length(100);
      });
      return describe('movement', function() {
        beforeEach(function() {
          this.marshal = new Piece({
            rank: '1',
            side: 0
          });
          this.scout = new Piece({
            rank: '9',
            side: 0
          });
          this.flag = new Piece({
            rank: 'B',
            side: 0
          });
          return this.bomb = new Piece({
            rank: 'B',
            side: 0
          });
        });
        it('should allow one space adjacent move not diagonal', function() {
          var board, from, invalidMoves, j, k, len, len1, move, results, to, validMoves;
          from = {
            x: 5,
            y: 5
          };
          validMoves = [
            {
              x: from.x + 1,
              y: from.y
            }, {
              x: from.x - 1,
              y: from.y
            }, {
              x: from.x,
              y: from.y - 1
            }, {
              x: from.x,
              y: from.y + 1
            }
          ];
          invalidMoves = [
            {
              x: from.x - 1,
              y: from.y + 1
            }, {
              x: from.x + 1,
              y: from.y - 1
            }, {
              x: from.x + 1,
              y: from.y + 1
            }, {
              x: from.x - 1,
              y: from.y - 1
            }, {
              x: from.x,
              y: from.y + 2
            }, {
              x: from.x + 2,
              y: from.y
            }
          ];
          for (j = 0, len = validMoves.length; j < len; j++) {
            to = validMoves[j];
            board = new Board;
            board.set(from, this.marshal);
            move = board.move(from, to);
            expect(move).to.equal(moveTypes.MOVE);
          }
          results = [];
          for (k = 0, len1 = invalidMoves.length; k < len1; k++) {
            to = invalidMoves[k];
            board = new Board;
            board.set(from, this.marshal);
            results.push(expect(function() {
              return board.move(from, to);
            }).to["throw"]());
          }
          return results;
        });
        it('should allow scouts to move straight in any direction', function() {
          var board, from, invalidMoves, j, k, len, len1, results, to, validMoves;
          from = {
            x: 5,
            y: 5
          };
          validMoves = [
            {
              x: from.x + 4,
              y: from.y
            }, {
              x: from.x - 5,
              y: from.y
            }, {
              x: from.x,
              y: from.y + 4
            }, {
              x: from.x,
              y: from.y - 5
            }
          ];
          invalidMoves = [
            {
              x: from.x + 4,
              y: from.y + 1
            }, {
              x: from.x - 5,
              y: from.y + 4
            }, {
              x: from.x - 2,
              y: from.y + 4
            }, {
              x: from.x + 4,
              y: from.y - 5
            }
          ];
          for (j = 0, len = validMoves.length; j < len; j++) {
            to = validMoves[j];
            board = new Board;
            board.set(from, this.scout);
            expect(function() {
              return board.move(from, to);
            }).to.not["throw"]();
          }
          results = [];
          for (k = 0, len1 = invalidMoves.length; k < len1; k++) {
            to = invalidMoves[k];
            board = new Board;
            board.set(from, this.marshal);
            results.push(expect(function() {
              return board.move(from, to);
            }).to["throw"]());
          }
          return results;
        });
        it('should not allow flags to move', function() {
          var board, from;
          from = {
            x: 5,
            y: 5
          };
          board = new Board;
          board.set(from, this.flag);
          return expect(function() {
            return board.move(from, {
              x: from.x,
              y: from.y + 1
            });
          }).to["throw"]();
        });
        it('should not allow bombs to move', function() {
          var board, from;
          from = {
            x: 5,
            y: 5
          };
          board = new Board;
          board.set(from, this.bomb);
          return expect(function() {
            return board.move(from, {
              x: from.x,
              y: from.y + 1
            });
          }).to["throw"]();
        });
        it('should not allow movement onto friendly piece', function() {
          var board, from;
          from = {
            x: 5,
            y: 5
          };
          board = new Board;
          board.set(from, this.marshall);
          board.set({
            x: from.x,
            y: from.y + 1
          }, this.scout);
          return expect(function() {
            return board.move(from, {
              x: from.x,
              y: from.y + 1
            });
          }).to["throw"]();
        });
        it('should not allow scouts to jump over pieces', function() {
          var board, from, i, inTheWayPositions, j, k, notInTheWayPositions, ref, ref1, results, toPositions;
          from = {
            x: 5,
            y: 5
          };
          toPositions = [
            {
              x: 9,
              y: 5
            }, {
              x: 2,
              y: 5
            }, {
              x: 5,
              y: 8
            }, {
              x: 5,
              y: 2
            }
          ];
          inTheWayPositions = [
            {
              x: 8,
              y: 5
            }, {
              x: 4,
              y: 5
            }, {
              x: 5,
              y: 6
            }, {
              x: 5,
              y: 3
            }
          ];
          notInTheWayPositions = [
            {
              x: 9,
              y: 6
            }, {
              x: 1,
              y: 5
            }, {
              x: 5,
              y: 9
            }, {
              x: 5,
              y: 0
            }
          ];
          for (i = j = 0, ref = toPositions.length; 0 <= ref ? j < ref : j > ref; i = 0 <= ref ? ++j : --j) {
            board = new Board;
            board.set(from, this.scout);
            board.setBlock(inTheWayPositions[i]);
            expect(function() {
              return board.move(from, toPositions[i]);
            }).to["throw"]();
          }
          results = [];
          for (i = k = 0, ref1 = toPositions.length; 0 <= ref1 ? k < ref1 : k > ref1; i = 0 <= ref1 ? ++k : --k) {
            board = new Board;
            board.set(from, this.scout);
            board.set(notInTheWayPositions[i], this.flag);
            results.push(expect(function() {
              return board.move(from, toPositions[i]);
            }).to.not["throw"]());
          }
          return results;
        });
        it('should not allow movement onto unmovable block', function() {
          var board, from;
          from = {
            x: 5,
            y: 5
          };
          board = new Board;
          board.set(from, this.marshal);
          board.setBlock({
            x: 6,
            y: 5
          });
          return expect(function() {
            return board.move(from, {
              x: 6,
              y: 5
            });
          }).to["throw"]();
        });
        return describe.only('attacking', function() {
          beforeEach(function() {
            this.from = {
              x: 5,
              y: 5
            };
            return this.to = {
              x: 5,
              y: 6
            };
          });
          describe('marshall', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_DRAW,
              '2': moveTypes.ATTACK_WON,
              '3': moveTypes.ATTACK_WON,
              '4': moveTypes.ATTACK_WON,
              '5': moveTypes.ATTACK_WON,
              '6': moveTypes.ATTACK_WON,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, marshal;
                  marshal = new Piece({
                    rank: '1',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, marshal);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('general', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_DRAW,
              '3': moveTypes.ATTACK_WON,
              '4': moveTypes.ATTACK_WON,
              '5': moveTypes.ATTACK_WON,
              '6': moveTypes.ATTACK_WON,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, general;
                  general = new Piece({
                    rank: '2',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, general);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('colonel', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_DRAW,
              '4': moveTypes.ATTACK_WON,
              '5': moveTypes.ATTACK_WON,
              '6': moveTypes.ATTACK_WON,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, colonel;
                  colonel = new Piece({
                    rank: '3',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, colonel);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('major', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_DRAW,
              '5': moveTypes.ATTACK_WON,
              '6': moveTypes.ATTACK_WON,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, major;
                  major = new Piece({
                    rank: '4',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, major);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('captain', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_DRAW,
              '6': moveTypes.ATTACK_WON,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, captain;
                  captain = new Piece({
                    rank: '5',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, captain);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('lieutenant', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_LOST,
              '6': moveTypes.ATTACK_DRAW,
              '7': moveTypes.ATTACK_WON,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, lieutenant;
                  lieutenant = new Piece({
                    rank: '6',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, lieutenant);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('sergeant', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_LOST,
              '6': moveTypes.ATTACK_LOST,
              '7': moveTypes.ATTACK_DRAW,
              '8': moveTypes.ATTACK_WON,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, sergeant;
                  sergeant = new Piece({
                    rank: '7',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, sergeant);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('miner', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_LOST,
              '6': moveTypes.ATTACK_LOST,
              '7': moveTypes.ATTACK_LOST,
              '8': moveTypes.ATTACK_DRAW,
              '9': moveTypes.ATTACK_WON,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.DISARM,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, miner;
                  miner = new Piece({
                    rank: '8',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, miner);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          describe('scout', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ATTACK_LOST,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_LOST,
              '6': moveTypes.ATTACK_LOST,
              '7': moveTypes.ATTACK_LOST,
              '8': moveTypes.ATTACK_LOST,
              '9': moveTypes.ATTACK_DRAW,
              'S': moveTypes.ATTACK_WON,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, scout;
                  scout = new Piece({
                    rank: '9',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, scout);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
          return describe('spy', function() {
            var result, results, rules, toRank;
            rules = {
              '1': moveTypes.ASSASINATION,
              '2': moveTypes.ATTACK_LOST,
              '3': moveTypes.ATTACK_LOST,
              '4': moveTypes.ATTACK_LOST,
              '5': moveTypes.ATTACK_LOST,
              '6': moveTypes.ATTACK_LOST,
              '7': moveTypes.ATTACK_LOST,
              '8': moveTypes.ATTACK_LOST,
              '9': moveTypes.ATTACK_LOST,
              'S': moveTypes.ATTACK_DRAW,
              'B': moveTypes.ATTACK_LOST,
              'F': moveTypes.CAPTURE
            };
            results = [];
            for (toRank in rules) {
              result = rules[toRank];
              results.push((function(toRank) {
                return it("should vs " + toRank + " " + MOVE_TYPE_TO_NAME[result], function() {
                  var board, spy;
                  spy = new Piece({
                    rank: 'S',
                    side: 0
                  });
                  board = new Board;
                  board.set(this.from, spy);
                  board.set(this.to, new Piece({
                    rank: toRank,
                    side: 1
                  }));
                  return expect(board.move(this.from, this.to)).to.equal(rules[toRank]);
                });
              })(toRank));
            }
            return results;
          });
        });
      });
    });
  });

}).call(this);
