# Moka-Rust
[![Build Status](https://travis-ci.org/AffogatoLang/Moka-Rust.svg?branch=master)](https://travis-ci.org/AffogatoLang/Moka-Rust)

## A Rust implementation of the Moka transpiler

Moka is a transpilation framework aimed at making it easier to develop domain specific languages in a short space of time. By defining
self contained pieces of language syntax that can be shared and reused freely, languages can be slotted together quickly. Moka
transpiles languages from one source format to another, meaning that no code is compiled or interpreted - the target of a Moka
language is the runtime of a different language.

A quick example of a great use of Moka would be to add pre-compile macros to a language that doesn't have that functionality (eg 
most interpreted languages), add the Moka compiler to the build phase of your project and distribute as if nothing were different.

### Installation
[[ No Current Distribution Available For Moka ]]

### Usage
Add Usage Here
```
Compile or run a Moka module
Usage:
    moka [-va] use <module> <input> <output>
    moka [-va] compile <module> <output>
    moka -h
    moka --version
Options:
    -a, --archive   The specified module is an archive instead of a folder
    -h, --help      Show this text
    -v, --verbose   Enable verbose output
    --version       Show the installed Moka version
```

### Building From Source
TBA
