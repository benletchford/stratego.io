import unittest

import models


class DataTest(unittest.TestCase):
    # enable the datastore stub
    nosegae_datastore_v3 = True

    def test_get_entity(self):
        """Naively tests that we can fetch an entity from the datastore"""
        entity = models.Game()
        entity.put()
        self.assertIsNotNone(entity)
        self.assertEqual(entity.red_hash, 'abc')
