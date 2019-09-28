# basic templater ![build status](https://travis-ci.com/luke-biel/bt.svg?branch=master)

Tool that allows you to store commonly used files and spawn them on demand in desired location (think sample configs, new projects etc.).
Currently tested only on OSX.

## Installation

Download binary or compile from source.  
Place in `/usr/local/bin`.
You are set to go.

## Usage

```
bt 0.1.0
   Automatic template file generator.
   
   USAGE:
       bt [config-location] <SUBCOMMAND>
   
   FLAGS:
       -h, --help       Prints help information
       -V, --version    Prints version information
   
   ARGS:
       <config-location>     [env: BT_HOME=]  [default: /Users/lukaszbiel/.bt]
   
   SUBCOMMANDS:
       help        Prints this message or the help of the given subcommand(s)
       new
       register
```
### `bt new`
#### To instantiate prepared template:
`bt new template_name`

#### To instantiate prepared template at desired location:
`bt new template_name tests/acceptance/new_test`

#### Additional flags
`new` command allows to specify `--max-depth` parameter when templates are folders with deep files tree.  
`--with-parent` flag controls if you want to keep folder structure of template when copying files; think - it will add `template_name/file.txt` instead of `file.txt`

Templates can be stored in sub folders, eg:  
`bt new cargo/web_app .`

###`bt register`
#### To create template from all files in current directory:  
`bt register template_name .`  
or  
`bt register template_name file.txt`  
to create template from given file.  
This command removes anything that existed previously as `template_name`

#### To append a file to an existing template:  
`bt register template_name . -a`

#### Additional flags
`register` command allows to specify `--max-depth` same as new.
