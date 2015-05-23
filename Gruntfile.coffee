module.exports = (grunt) ->

  grunt.loadNpmTasks 'grunt-gae'
  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-less'
  grunt.loadNpmTasks 'grunt-contrib-clean'
  grunt.loadNpmTasks 'grunt-contrib-requirejs'
  grunt.loadNpmTasks 'grunt-mocha-phantomjs'

  grunt.initConfig
    pkg: grunt.file.readJSON 'package.json'

    clean:
      build:
        ['build']
      app:
        ['js/**/*.js']
      tests:
        ['test/specs/**/*.js']

    less:
      app:
        options:
          compress: true
        files:
          'build/styles.min.css': 'css/styles.less'

    coffee:
      app:
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

    requirejs:
      app:
        options:
          baseUrl: './js',
          name: './main'
          out: 'build/stratego.min.js'
          paths:
            'jquery': 'http://ajax.googleapis.com/ajax/libs/jquery/1.11.1/jquery.min'

    mocha_phantomjs:
      options:
        reporter: 'spec'
      all: ['test/index.html']

  grunt.registerTask 'build', [
    'clean:app'
    'coffee:app'
    'less'
    'requirejs'
  ]

  grunt.registerTask 'test', [
    'clean:tests'
    'coffee:tests'
    'mocha_phantomjs'
  ]
