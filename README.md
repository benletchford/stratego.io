www.stratego.io
======
Multiplayer HTML5 [Stratego](https://en.wikipedia.org/wiki/Stratego).

![Stratego.io preview](/preview.png)

Built upon [Pusher](https://pusher.com/) (for websockets) and Google App Engine.

In theory should work on anything that has a browser!

While this only supports the most basic gameplay, hopefully it'll get a lot better with time.

Contributing
======
Contrubutions are always welcome.

To setup your local instance you'll need the [Google App Engine SDK](https://cloud.google.com/appengine/downloads?hl=en).

You can install all the dependencies by doing:

    $ npm install
    $ bower install

You can run the front-end tests with:

    $ grunt build:tests test

You can run the back-end tests with (uses nosegae):

    $ pynt test

You can build the app with:

    $ grunt build

You can build the graphics with:

    $ grunt build:graphics

You can deploy the app to App Engine by doing (the grunt task will look for a gae.auth file which you must create yourself, see [grunt-gae](https://github.com/maciejzasada/grunt-gae)):

    $ grunt deploy

You'll need to create `js/PUSHER_CREDENTIALS.coffee`, eg:

```
define (require) ->

  KEY: "YOUR_PUSHER_KEY"
```

You'll need to create `gae/CONSTANTS/PUSHER_CREDENTIALS.py`, eg:

```
APP_ID = "YOUR_APP_ID"
KEY = "YOUR_KEY"
SECRET = "YOUR_SECRET"
```

Piece Graphics
======
Pieces found at [vector.gissen.nl](http://vector.gissen.nl/stratego.html)

Issues
======
Any bugs or issues, please open an issue through github.
