# Overview

A tool for creating bash/zsh completions

# bash

```
complete -o bashdefault -o default -o nospace -C ucompleter dumper
```

Will look for a dumper.yaml file within directories specified by UCOMPLETER_PATH

# zsh

# Environment vars

|   Var          | Effect                                                                                    |
|----------------|-------------------------------------------------------------------------------------------|
| UCOMPLETER_PATH| Paths to search for configurations.  Colon Separated.  Default: .:$HOME/.config/ucompleter|
|                |                                                    

