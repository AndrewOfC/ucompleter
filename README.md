# Overview

A tool for creating bash/zsh completions

# bash

```
complete -o bashdefault -o default -o nospace -C ucompleter dumper
```

Will look for a dumper.yaml file within directories specified by UCOMPLETER_PATH

# zsh
(work in progress)
```

_register_tool() {
    local -a args
    args=($(_call_program current ucompleter register_tool "$words[CURRENT]"))
    _describe 'commands' args
    return 0
}

compdef _register_tool register_tool

```

# Environment vars

|   Var          | Effect                                                                                    |
|----------------|-------------------------------------------------------------------------------------------|
| UCOMPLETER_PATH| Paths to search for configurations.  Colon Separated.  Default: .:$HOME/.config/ucompleter|                                                    

# completion-metadata

If a 'completion-metadata' key is found at the top level of a configuration file, it will be used to effect the completion.

| key      | purpose                                                                                                     |
|----------|-------------------------------------------------------------------------------------------------------------|
| root     | completions will be taken from the hash specified by this key. If not specified, the whole document is used |      
| terminus | finding this key in a hash will signal that the terminal completion has been reached.                       |

## Example

```yaml
completion-metadata:
  root: registers
  terminus: offset

registers:
  Peripherals:
    GPIO1:
      - description: GPIO pin
        offset: 0x0000
```
Completion will go down as far as Peripherals.GPIO1 