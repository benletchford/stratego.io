module.exports = (grunt) ->

  grunt.loadNpmTasks 'grunt-gae'
  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-less'
  grunt.loadNpmTasks 'grunt-contrib-clean'
  grunt.loadNpmTasks 'grunt-contrib-requirejs'
  grunt.loadNpmTasks 'grunt-mocha-phantomjs'
  grunt.loadNpmTasks 'grunt-contrib-htmlmin'

  grunt.initConfig
    pkg: grunt.file.readJSON 'package.json'

    clean:
      app:
        ['app/static']
      js:
        ['js/**/*.js']
      tests:
        ['test/specs/**/*.js']

    less:
      app:
        options:
          compress: true
        files:
          'app/static/stratego.min.css': 'css/main.less'

    coffee:
      js:
        expand: true
        flatten: true
        src: [
          'js/**/*.coffee'
        ]
        dest: 'js'
        ext: '.js'
      tests:
        expand: true
        flatten: true
        src: [
          'test/specs/**/*.coffee'
        ]
        dest: 'test/specs'
        ext: '.spec.js'

    htmlmin:
      app:
        options:
          removeComments: true
          collapseWhitespace: true
          link: true
          minifyJS: true
        files:
          'app/static/index.html': 'html/index.html'

    requirejs:
      app:
        options:
          baseUrl: './js',
          name: './main'
          out: 'app/static/stratego.min.js'
          paths:
            'jquery'    : 'empty:'
            'backbone'  : 'empty:'
            'underscore': 'empty:'
            'pace'      : 'empty:'

    mocha_phantomjs:
      options:
        reporter: 'spec'
      all: ['test/index.html']

    gae:
      deploy:
        options:
          path: 'app'
          auth: 'gae.auth'
          version: '1'
        action: 'update'

  grunt.registerTask 'build', [
    'clean:app'
    'clean:js'
    'coffee:js'
    'less'
    'htmlmin'
    'requirejs'
  ]

  grunt.registerTask 'test', [
    'clean:tests'
    'coffee:tests'
    'mocha_phantomjs'
  ]

  grunt.registerTask 'deploy', [
    'gae'
  ]
