require.config({
  baseUrl: '../js',
  paths: {
    // 'jquery': 'http://ajax.googleapis.com/ajax/libs/jquery/1.11.1/jquery.min',
    'mocha': '../node_modules/mocha/mocha',
    'chai' : '../node_modules/chai/chai',
  },
  shim: {
    mocha: {
      exports: 'mocha'
    }
  }
});

define(function(require) {
  var chai = require('chai');
  var mocha = require('mocha');

  // Chai
  expect = chai.expect;
  // chai.use(chaiJquery);

  mocha.setup('bdd');

  require([
    '../test/specs/Board.spec',
    '../test/specs/Piece.spec',
  ], function(require) {
    if (window.mochaPhantomJS) {
        mochaPhantomJS.run();
    }
    else {
        mocha.run();
    }
  });
});
