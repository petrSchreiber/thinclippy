# thinClippy - thinBasic script code analyzer
[![Build Status](https://travis-ci.com/petrSchreiber/thinclippy.svg?branch=master)](https://travis-ci.com/petrSchreiber/thinclippy)

The _thinClippy_ is an optional thinBasic tool to check the thinBasic script source code and provide valuable hints.

## Currently supported checks
* formal check of `#compiled/#endcompiled` specification

## How to build from code
You will need Rust programming language to compile the tool.

### Installing Rust
* Download and install the [Build Tools for Visual Studio](https://www.visualstudio.com/cs/downloads/?q=Build+Tools+for+Visual+Studio).
* [Install Rust](https://www.rust-lang.org/en-US/install.html) via Rustup. **Please** customize the installation to ensure 32 bit pipeline currently needed by ThinBASIC:
  * Run the rustup-init.exe
  * Press *2* to alter the default settings
  * For *Default host triple?* enter *i686-pc-windows-msvc*
  * For *Default toolchain?* enter *stable*
  * For *Modify PATH variable?* enter *y*
  * Proceed with installation with *1*

## How to use
The _thinClippy_ is a command-line tool.

Please run `thinclippy.exe --help` to see all the options.
