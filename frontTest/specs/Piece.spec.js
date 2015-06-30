(function() {
  define(function(require) {
    var Piece, ranks;
    Piece = require('Piece');
    ranks = require('ranks');
    return describe('Piece', function() {
      it('should not throw if rank is valid', function() {
        var rank, results;
        results = [];
        for (rank in ranks) {
          results.push(expect(function() {
            return new Piece({
              rank: rank,
              side: 0
            });
          }).to.not["throw"]());
        }
        return results;
      });
      it('should throw if rank is not valid', function() {
        return expect(function() {
          return new Piece({
            rank: 'abc',
            side: 0
          });
        }).to["throw"]();
      });
      it('should not throw if side 0 or 1', function() {
        expect(function() {
          return new Piece({
            rank: '1',
            side: 0
          });
        }).to.not["throw"]();
        return expect(function() {
          return new Piece({
            rank: '1',
            side: 1
          });
        }).to.not["throw"]();
      });
      return it('should throw if side not valid', function() {
        expect(function() {
          return new Piece({
            rank: '1',
            side: 2
          });
        }).to["throw"]();
        return expect(function() {
          return new Piece({
            rank: '1',
            side: -1
          });
        }).to["throw"]();
      });
    });
  });

}).call(this);
