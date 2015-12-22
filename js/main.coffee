define (require) ->

  Router = require './Router'

  require '../css/main.less'

  appReady = ->
    $(document.body).css 'background-color', '#5EB549'

    new Router()
    Backbone.history.start()

  # Due to a potential race condition Pace could finish before the hide event
  # is bound and it will never be triggered :(
  if Pace.running is false
    appReady()
  else
    Pace.on 'hide', appReady
