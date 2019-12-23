[![Build Status](https://travis-ci.org/luke-biel/bts.svg?branch=master)](https://travis-ci.org/luke-biel/bts)

# bts

Command line utility to create simple file snippets available to be instantiated at any time in future

It allows to **register** and **spawn** snippets at will

## Installation
`cargo install bts`

## Usage
### NEW
Instantiate copy of an existing snippet

`bts new SOURCE [DESTINATION] [-w/--with-parent] [-m/--max-depth _]`

#### **SOURCE**
- is snippet name.

It's good idea to remember that for the time being snippets are stored in directories.
Therefore nesting is allowed and advised.
For example,
`config/mysql`, `config/psql`, `config/sqlite` are good examples of template names
and
`config_mysql`, `config_psql`, `config_sqlite` are, while correct, discouraged.
But hey, these are your snippets. Consider it only to be an advice.

#### **DESTINATION**
- is target folder name

Place where you want to instantiate a snippet. By default `pwd` is used.

#### **WITH-PARENT**
- defines whether folder should be spawned preserving snippet name

This means that `bts basic/template01 -w` will spawn files in `./basic/template01/` instead of `.`

#### **MAX-DEPTH**
- how deep the copy should go

Copies snippet only until given depth is reached in directory tree.
For example,
```
basic/template01/
                |- file.txt
                |-/ subdir

bts new basic/template01 -m 1
```
will produce only file.txt in current directory.
This parameter accepts numbers in range 0..255, default value is 32.

### Register
Create new snippet from files

`bts register TEMPLATE_NAME SOURCE [-a/--append] [-m/--max-depth]`

#### **TEMPLATE_NAME**
- is snippet name

It clears previous snippets stored at given namespace, so calling `bts basic .` will also remove `basic/template01`.

#### **SOURCE**
- is a path to snippet files

When **SOURCE** is a file, this file will be stored at snippet namespace.
When **SOURCE** is a directory, all contents of that directory will be stored at snippet namespace.

#### **APPEND**
- defines whether we want to append to existing snippet

This allows to create snippet only from selected files in directory.

#### **MAX-DEPTH**
- defines how deep should `bts` search for files when creating a snippet.

Accepts numbers between 0 and 255, default value is 32.

## Contribution
I may accept new features, but that will only happen if I can see that it's useful.
It's better to create issue at github before attempting to implement something.
Bug fixes are always welcome.
You can look into `.travis.yml` for build steps, but tldr is that I will merge only features that pass
`cargo clippy --all-targets --all-features -- -D warnings`
and
`cargo fmt --all -- --check`

## TBD
* provide GUI for the app (?)
* default snippets (sample rust projects etc.)
* pass-through for selected applications (eg. `bts spawn cargo/bin` calls `cargo new --bin`)

License: MIT
