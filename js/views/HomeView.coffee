define (require) ->

  PanelLinkView = require '../panel/PanelLinkView'

  class extends Backbone.View
    className: 'home-view panel'

    initialize: ->
      @$el.append (new PanelLinkView
        title: 'Online Stratego'
        description: 'Get matched with someone online.'
        href: '#setup/pool').el

      @$el.append (new PanelLinkView
        title: 'Play with a friend'
        description: 'Start a private game with a friend.'
        href: '#setup/create').el
