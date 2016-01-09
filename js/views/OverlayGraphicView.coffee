define (require) ->

  IMAGES =
    tree:
      tree1:
        baseWidth : 49
        baseHeight: 54
        maxScale  : 2
        minScale  : 0.5
      tree2:
        baseWidth : 49
        baseHeight: 53
        maxScale  : 2
        minScale  : 0.5
      tree3:
        baseWidth : 49
        baseHeight: 53
        maxScale  : 2
        minScale  : 0.5

    grass:
      grass1:
        baseWidth : 24
        baseHeight: 20
        maxScale  : 2
        minScale  : 0.5
      grass2:
        baseWidth : 16
        baseHeight: 11
        maxScale  : 2
        minScale  : 0.5

  class extends Backbone.View
    className:  'overlay-graphic-view'

    initialize: (rect, kind) ->
      keys = Object.keys IMAGES[kind]
      randomKey = keys[keys.length * Math.random() << 0]

      image = IMAGES[kind][randomKey]

      width  = image.baseWidth
      height = image.baseHeight

      if rect.bottom > rect.right
        if rect.top isnt 0
          top = (rect.bottom - rect.top - height) * Math.random() << 0
          top += rect.top
        else
          top = (rect.bottom - height) * Math.random() << 0

        left = (rect.right - width) * Math.random() << 0

      else
        if rect.left isnt 0
          left = (rect.right - rect.left - width) * Math.random() << 0
          left += rect.left
        else
          left = (rect.right - width) * Math.random() << 0

        top = (rect.bottom - height) * Math.random() << 0

      @$el.addClass "image-#{randomKey}"
      @$el.css 'width', width
      @$el.css 'height', height
      @$el.css 'marginLeft', left
      @$el.css 'marginTop', top

      rule = 'rotate(' + (360 * Math.random() << 0) + 'deg)'
      @$el.css
        '-webkit-transform': rule
        '-moz-transform': rule
        '-ms-transform': rule
        'transform' : rule
