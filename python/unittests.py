import io
import os
import sys
import unittest

import yaml

from python.ucompleter import UCompleter


class CompletionsTest(unittest.TestCase):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        base = os.path.dirname(__file__)
        with open(os.path.join(base, '..', 'aep_rust_common', 'test_data.yaml'), 'r') as f:
            self._root = yaml.safe_load(f)

    def input_output_check(self, input, output):
        s = io.StringIO()
        comp = UCompleter(self._root)
        path = comp.write_completions(input, s)
        self.assertEqual(s.getvalue(), output)
        return comp, path

    def test_level(self):
        self.input_output_check('level1.level2.',
                                'level1.level2[0]\nlevel1.level2[1]\n')

    # ported from unittests.rs
    def test_empty(self):
        self.input_output_check('',
                                'GPIO\narray\nlevel1\nlevel1b\nlevel1c\nulevel\nxlevel\n')

    def test_one_path(self):
        self.input_output_check('l',
                                'level1\nlevel1b\nlevel1c\n')

    def test_array1(self):
        self.input_output_check('array',
                                'array[0]\narray[1]\narray[2]\n')

    def test_array2(self):
        self.input_output_check('array[1]',
                                '')
    def test_array3(self):
        self.input_output_check('array[',
                                'array[0]\narray[1]\narray[2]\n')

    def test_level_drop(self):
        self.input_output_check('level1.level2',
                                "level1.level2\nlevel1.level2a\nlevel1.level2b\n")
        self.input_output_check('level1.level2a',
                                '') # todo check rust version

    def test_gpio(self):
        self.input_output_check('G',
                                'GPIO.pins\nGPIO.words\n')

    def test_gpio_p(self):
        self.input_output_check('GPIO.p',"GPIO.pins[0]\nGPIO.pins[1]\n")

    def test_gpio_pin0(self):
        self.input_output_check('GPIO.pins[0].',
                                "GPIO.pins[0].clear\nGPIO.pins[0].function\nGPIO.pins[0].level\nGPIO.pins[0].set\n")

    def test_descending(self):
        comp, path = self.input_output_check("ulev",
                                "")
        self.assertEqual(comp[path], "foo")

if __name__ == '__main__':
    unittest.main()
