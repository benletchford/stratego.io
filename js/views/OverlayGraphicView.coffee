define (require) ->

  IMAGES =
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

    initialize: (rect) ->
      keys = Object.keys IMAGES
      randomKey = keys[keys.length * Math.random() << 0]

      image = IMAGES[randomKey]

      width  = image.baseWidth
      height = image.baseHeight

      rect.right -= width
      rect.bottom -= height

      debugger
      left = rect.right * Math.random() << 0
      top  = rect.bottom * Math.random() << 0

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
