_       = require 'underscore'
webpack = require 'webpack'

webpackBase =
  module:
    loaders: [
        test: /\.css$/, loader: 'style!css'
      ,
        test: /\.less$/, loader: "style!css!less"
      ,
        test: /\.coffee$/, loader: 'coffee-loader'
      ,
        test: /\.(jpe?g|png|gif|svg)$/i, loaders: [
            'file?hash=sha512&digest=hex&name=[hash].[ext]',
            'image-webpack?bypassOnDebug&optimizationLevel=0&interlaced=false'
        ]
      ,
        test: /\.jade$/, loader: 'jade-loader'
    ]
  resolve:
    root: [
      'js'
    ]
    extensions: [
      '.js'
      '.coffee'
      ''
    ]
  externals:
    jquery    : 'jQuery'
    backbone  : 'Backbone'
    underscore: '_'
    pusher    : 'Pusher'
  plugins: [
    new webpack.ProvidePlugin({
        $: 'jquery',
        jQuery: 'jquery',
        'window.jQuery': 'jquery'
    })
  ]

module.exports = (grunt) ->
  grunt.loadNpmTasks 'grunt-webpack'
  grunt.loadNpmTasks 'grunt-gae'
  grunt.loadNpmTasks 'grunt-mocha-phantomjs'
  grunt.loadNpmTasks 'grunt-contrib-htmlmin'
  grunt.loadNpmTasks 'grunt-contrib-clean'

  grunt.initConfig
    pkg: grunt.file.readJSON 'package.json'

    clean: ['js/**/*.js']

    htmlmin:
      app:
        options:
          removeComments: true
          collapseWhitespace: true
          link: true
          minifyJS: true
        files:
          'app/static/index.html': 'html/index.html'

    mocha_phantomjs:
      options:
        reporter: 'dot'
      all: ['test/index.html']

    gae:
      deploy:
        options:
          path: 'app'
          auth: 'gae.auth'
          version: '1'
        action: 'update'

    webpack:
      app: _.extend({
          entry: './js/main.coffee'
          output:
            path: __dirname + '/app/static'
            filename: 'stratego.js'
        }, webpackBase)

      tests: _.extend({
          entry: 'mocha!./test/specRunner.js'
          output:
            path: __dirname + '/test'
            filename: 'testBundle.js'
        }, webpackBase)

  grunt.registerTask 'build', [
    'clean'
    'htmlmin'
    'webpack:app'
  ]

  grunt.registerTask 'build:tests', [
    'webpack:tests'
  ]

  grunt.registerTask 'deploy', [
    'gae'
  ]

  grunt.registerTask 'test', [
    'mocha_phantomjs'
  ]
