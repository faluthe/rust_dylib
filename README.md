# Rust Dynamic Library

A basic Rust library for Linux, injectable via `gdb`. Upon injection a constructor will run and print a `Hello world!` message. Upon uninjection a destructor will run and print a `Goodbye world!` message.

## Try it out

First, build the library:

```bash
cargo build
```

Then, launch `tail` (or any other program with easy STDOUT access):

```bash
tail
```

From another terminal, inject with `gdb` via the inject script:

```bash
sudo bash so_inject.sh inject $(pidof tail) /full/path/to/librust_dylib.so
```
*Notice `sudo` permissions must be used and the full path to the library must be provided*

The terminal running `tail` should print the constructor's message:

```bash
$ tail
Hello, world!
```

From another terminal, uninject with `gdb` via the inject script:

```bash
sudo bash so_inject uninject $(pidof tail) 0xffffffffffff
```
*Notice you MUST provide the script the handle of the loaded library, returned from `gdb` during injection*

The terminal running `tail` should print the destructor's message:

```bash
$ tail
Hello world!
Goodbye world!
```

## Why it works

The `#[link_section = ".init_array"]` and `#[link_section = ".fini_array"` attributes place the function pointers into special sections that the dynamic loader automatically calls when loading (`.init_array`) and unloading (`.fini_array`) a shared object on Linux. The `#[used]` attribute ensures the compiler keeps these symbols around.
