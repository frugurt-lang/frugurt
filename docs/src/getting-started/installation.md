# Installation and Running

## Prebuilt executable

Prebuilt executable is only available for Windows.

You can download if from [release page](https://github.com/leokostarev/frugurt-lang/releases).

## Build from source code

You can build Frugurt from [source code](https://github.com/leokostarev/frugurt-lang) on any platform.


Use [Rust Toolchain](https://www.rust-lang.org/tools/install) to build interpreter.  
Interpreter code is located in `rfruit` directory.  
Building rust executable is straightforward.
  
Use [Haskell Tool Stack](https://docs.haskellstack.org/en/stable/) to build the converter.  
Converter code is located in `converter` directory.  
Run `stack build` to build the converter.

Place `converter.exe` in the same directory as `rfruit.exe`