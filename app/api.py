import json

import webapp2
from lib.firebase.wrapper import Firebase


class MainPage(webapp2.RequestHandler):

    def get(self):

        ref = Firebase('https://shining-fire-2321.firebaseio.com/something.json',
                       'u3bOjyHvk2Pc3a42YoqQBiVrPb7T97Hu7UIxDO5I')

        response = ref.put({
            'firstname': 'Ben',
            'lastname': 'Letchford'
        })

        self.response.headers['Content-Type'] = 'text/plain'
        self.response.write(json.dumps(response))


app = webapp2.WSGIApplication([
    ('/api/hi', MainPage)
], debug=True)
