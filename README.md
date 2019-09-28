# basic templates ![build status](https://travis-ci.com/luke-biel/bt.svg?branch=master)

Tool that allows you to store commonly used files and spawn them on demand in desired location (think sample configs, new projects etc.).
Currently tested only on OSX.

## Installation

`cargo install bts`  
or  
Download binary/compile from source.  
Place in `/usr/local/bin`.  
You are set to go.

## Usage

```
bts 0.1.0
   Automatic template file generator.
   
   USAGE:
       bts [config-location] <SUBCOMMAND>
   
   FLAGS:
       -h, --help       Prints help information
       -V, --version    Prints version information
   
   ARGS:
       <config-location>     [env: BT_HOME=]  [default: /Users/lukaszbiel/.bts]
   
   SUBCOMMANDS:
       help        Prints this message or the help of the given subcommand(s)
       new
       register
```
### `bts new`
#### To instantiate prepared template:
`bts new template_name`

#### To instantiate prepared template at desired location:
`bts new template_name tests/acceptance/new_test`

#### Additional flags
`new` command allows to specify `--max-depth` parameter when templates are folders with deep files tree.  
`--with-parent` flag controls if you want to keep folder structure of template when copying files; think - it will add `template_name/file.txt` instead of `file.txt`

Templates can be stored in sub folders, eg:  
`bts new cargo/web_app .`

### `bts register`
#### To create template from all files in current directory:  
`bts register template_name .`  
or  
`bts register template_name file.txt`  
to create template from given file.  
This command removes anything that existed previously as `template_name`

#### To append a file to an existing template:  
`bts register template_name . -a`

#### Additional flags
`register` command allows to specify `--max-depth` same as new.
