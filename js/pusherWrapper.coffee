# Returns a singleton Wrapper that has some util functions for Pusher

define (require) ->

  instance = null

  class PusherWrapper
    constructor: ->
      @connectionPromise = $.Deferred()

    connect: ->
      if not @pusher
        @connectionPromise = $.Deferred()

        @pusher = new Pusher 'fd2e668a4ea4f7e23ab6',
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
