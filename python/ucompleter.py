import os
import re
import sys

import yaml

KEY_MATCH = 0
INDEX_MATCH = 1
PERIOD_MATCH = 2

class UCompleter:
    def __init__(self, root):
        """
        :param root: dict or list from yaml or json
        """
        # todo zsh parser
        self._re = re.compile(r"(?:([^.\[\]\\]+)|(?:\[(\d+)]?))(\.)?")

        if isinstance(root, dict) and 'completion-metadata' in root:
            self._root = root[root['completion-metadata']['root']]
            if 'terminal-fields' in root['completion-metadata']:
                self._terminal_fields = set(root['completion-metadata']['terminal-fields'])
        else:
            self._root = root

        return

    def __getitem__(self, path):
        """
        Access nested elements using path string
        :param path: String path like 'level1.level2' or 'array[0]'
        :return: Value at the specified path
        """
        current = self._root
        matches = self._re.findall(path)

        for match in matches:
            key = match[KEY_MATCH]
            index = match[INDEX_MATCH]

            if isinstance(current, dict) and key:
                if key not in current:
                    raise KeyError(f"Key '{key}' not found")
                current = current[key]
            elif isinstance(current, list) and index:
                idx = int(index)
                if idx >= len(current):
                    raise IndexError(f"Index {idx} out of range")
                current = current[idx]

        return current

    def has_terminal_field(self, current) -> bool:
        if not isinstance(current, dict):
            return False
        for key in current.keys():
            if key in self._terminal_fields:
                return True
        return False

    def sep(self, current, empty_path, last):
        if empty_path or last:
            return ''
        if isinstance(current, dict):
            return '.'
        return ''

    def keys_starting_with(self, key, current:dict):
        return sorted(filter(lambda x: x.startswith(key), current.keys()))

    def write_completions(self, path, strm):
        completions = []
        current_path = ''
        current = self._root
        empty_path = True
        matches = self._re.findall(path)
        for i, match in enumerate(matches):
            last = i == (len(matches) - 1)
            terminated = match[PERIOD_MATCH]
            key = match[KEY_MATCH]
            while True:
                if isinstance(current, dict):
                    if terminated:
                        current = current[key]
                        current_path += key
                        empty_path = False
                        current_path += self.sep(current, empty_path, last)
                        if not last:
                            break
                        key = ""
                        terminated = False
                        continue

                    keys = self.keys_starting_with(key, current)
                    if not keys:
                        return ''
                    if len(keys) == 1:
                        current = current[keys[0]]
                        current_path += keys[0]
                        if self.has_terminal_field(current):
                            return current_path
                        empty_path = False
                        key = ''
                        current_path += self.sep(current, empty_path, last)
                        break

                    for key in keys:
                        strm.write(f"{current_path}{key}\n")
                    return ''

                if isinstance(current, list):
                    if len(current) == 1:
                        current_path += "[0]" # todo apply array parser port
                        empty_path = False
                        current = current[0]
                        continue

                    index = match[INDEX_MATCH]
                    if not index:
                        for i in range(len(current)):
                            empty_path = False
                            index_str = f"[{i}]" # todo apply array parser port
                            current_path += self.sep(current, empty_path, last)
                            strm.write(f"{current_path}{index_str}\n")
                        return ''
                    index = int(index, 0)
                    if index >= len(current):
                        return
                    current = current[index]
                    current_path += f"[{index}]"
                    current_path += self.sep(current, empty_path, last)
                    empty_path = False
                    break
                return current_path # scalar, we've bottomed out

        if isinstance(current, dict):
            if self.has_terminal_field(current):
                return current_path
            for key in sorted(current.keys()):
                strm.write(f"{current_path}.{key}\n")
            return current_path

        if isinstance(current, list):
            for i in range(len(current)):
                strm.write(f"{current_path}[{i}]\n")
        return current_path
