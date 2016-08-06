# Returns a singleton Wrapper that has some util functions for Pusher

define (require) ->
  PUSHER_CREDENTIALS = require './PUSHER_CREDENTIALS'

  instance = null

  class PusherWrapper
    constructor: ->
      @connectionPromise = $.Deferred()

    connect: ->
      if not @pusher
        @connectionPromise = $.Deferred()

        @pusher = new Pusher PUSHER_CREDENTIALS.KEY,
          encrypted: true
          authEndpoint: '/api/pusher/auth'

        @pusher.connection.bind 'connected', =>
          @connectionPromise.resolve()

      @connectionPromise

    unsubscribeAll: (exceptedChannelNames = []) ->
      if @pusher
        channels = @pusher.allChannels()

        for channel in channels
          if exceptedChannelNames.indexOf(channel.name) > -1 then continue

          channel.unbind()
          @pusher.unsubscribe(channel.name)

  instance ?= new PusherWrapper()
