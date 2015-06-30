define(function(require) {
  var chai = require('chai');

  // Make expect global :-|
  expect = chai.expect;

  require([
    'mocha!./specs/Game.spec.coffee',
    'mocha!./specs/Setup.spec.coffee'
  ]);
});	