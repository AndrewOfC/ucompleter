import os
import sys
import unittest

import yaml

from python.ucompleter import UCompleter


class MyTestCase(unittest.TestCase):
    def test_something(self):
        base = os.path.dirname(__file__)

        with open(os.path.join(base, '..', 'aep_rust_common', 'test_data.yaml'), 'r') as f:
            root = yaml.safe_load(f)

            comp = UCompleter(root)
            comp.write_completions('level1.level2.', sys.stdout)

        self.assertEqual(True, False)  # add assertion here


if __name__ == '__main__':
    unittest.main()
