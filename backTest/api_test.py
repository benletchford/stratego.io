import unittest

import models


class ApiTests(unittest.TestCase):
    # enable the datastore stub
    nosegae_datastore_v3 = True

    def test_hashes_are_created(self):
        game = models.Game()
        game.put()

        for _hash in [game.red_hash, game.blue_hash, game.join_hash]:
            self.assertEqual(len(_hash), 6)
            self.assertIsInstance(_hash, unicode)
