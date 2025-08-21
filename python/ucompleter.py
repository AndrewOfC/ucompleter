

import re

import yaml

KEY_MATCH = 1
PERIOD_MATCH = 2
INDEX_MATCH = 3

class UCompleter:
    def __init__(self, root):
        """
        :param root: dict or list from yaml or json
        """
        # todo zsh parser
        self._re = re.compile(r"([^.\[\]\\]+)(\.)?|(?:\[(\d+)]?)?")

        if isinstance(root, dict) and 'completion-metadata' in root:
            self._root = root[root['completion-metadata']['root']]
            if 'terminal-fields' in root['completion-metadata']:
                self._terminal_fields = set(root['completion-metadata']['terminal-fields'])
        else:
            self._root = root

        return

    def has_terminal_field(self, current) -> bool:
        if not isinstance(current, dict):
            return False
        for key in current.keys():
            if key in self._terminal_fields:
                return True
        return False

    def sep(self, current, empty_path):
        if empty_path:
            return ''
        if isinstance(current, dict):
            return '.'
        return ''

    def keys_starting_with(self, key, current:dict):
        return list(filter(lambda x: x.startswith(key), current.keys()))

    def write_completions(self, path, strm):
        completions = []
        current_path = ''
        current = self._root
        empty_path = True
        for match in self._re.finditer(path):
            terminated = match.group(PERIOD_MATCH)
            key = match.group(1)

            if isinstance(current, dict):
                if key not in current:
                    return
                if terminated:
                    current = current[key]
                    current_path += key
                    empty_path = False
                    current_path += self.sep(current, empty_path)
                    if self.has_terminal_field(current):
                        return
                    continue

            keys = self.keys_starting_with(key, current)
            if not keys:
                return
            if len(keys) == 1:
                current = current[keys[0]]
                current_path += keys[0]
                current_path += self.sep(current, empty_path)
                empty_path = False
                continue

            for key in keys:
                strm.write(f"{current_path}{key}\n")
                return

            if isinstance(current, list):
                if len(current) == 1:
                    current_path += "[0]" # todo apply array parser port
                    empty_path = False
                    current = current[0]
                    continue

                index = match.group(INDEX_MATCH)
                if index is None:
                    for i in range(len(current)):
                        current_path += f"[{i}]"
                        empty_path = False
                        strm.write(f"{current_path}\n")
                    return
                index = int(index, 0)
                if index >= len(current):
                    return
                current = current[index]
                current_path += f"[{index}]"
                current_path += self.sep(current, empty_path)
                empty_path = False
                continue
            break # scalar, we've bottomed out


        return

def main():
    import yaml
    base = os.path.dirname(__file__)

    with open(os.path.join(base, '..', 'aep_rust_common', 'test_data.yaml'), 'r') as f:
        root = yaml.safe_load(f)





    return

if __name__ == "__main__":
    main()