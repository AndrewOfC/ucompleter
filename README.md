# Overview

A tool for creating bash/zsh completions for command line programs.

Given a configuration file written in yaml(and perhaps json) where sections and fields within the file 
are used as parameters, this tool can provide quick and easy navigation.

# Addressing
* Hash fields are separated with a '.'
* In bash Array members are separated with a [i] with i as a 0 based index.  
* In zsh arrays are derefenced with a @i with i as a 0 based index.

(NOTE:  I would love to use [] as array delimiters for both, but zsh isn't having it)

## Example

```yaml
field1:
  array:
    - member
    - member2
    - field1a: data
      field2a: data2
        

```
to access 'data' you could type:
* "f &lt;TAB&gt;" ucompleter will provide "field." to bash
* "field1.&lt;TAB&gt;" ucompleter will provide "field.array"
* "field.array&lt;TAB&gt;" will provide: "field.array[0] field.array[1] field.array[2]"
* "field.array[2]"

# bash

```
complete -o bashdefault -o default -o nospace -C ucompleter <target_app>
```

Will look for a dumper.yaml file within directories specified by &lt;TARGET_APP&gt;_PATH

# zsh

```
_register_tool_complete() {
  local -a regs
  zstyle ':completion:*' add-space false
  regs=( "${(@f)"$(ucompleter -z register_tool $words[2])"}" )
  compadd -a regs
}

compdef _register_tool_complete register_tool
```

# Environment vars

| Var                | Effect                                                                                         |
|--------------------|------------------------------------------------------------------------------------------------|
| <TARGET_APP>_PATH  | Paths to search for configurations.  Colon Separated.<br>Default: ./:$HOME/.config/ucompleter/ |                                                    
| UCOMPLETER_VERBOSE | Setting this to > 0 will have ucompleter echo the command line arguments it's been given       |

# completion-metadata

If a 'completion-metadata' key is found at the top level of a configuration file, it will be used to effect the completion.

| key             | purpose                                                                                                                                             |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| root            | completions will be taken from the hash specified by this key. If not specified, the whole document is used                                         |      
| terminal-fields | finding this key in a hash will signal that the specific record has been reached.   At this point, the node should provide the target_app with data |



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
Completion will go down as far as Peripherals.GPIO1[0] 

# Building

## aarch64-unknown-linux-gnu

Suitable for executing on a raspberrypi4b, or ubuntu

```bash
rustup target add aarch64-unknown-linux-gnu # once
cargo build 



```