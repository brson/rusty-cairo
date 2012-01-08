## Synopsis

This project contains bindings of the [cairo vector graphics](http://cairographics.org) library to the system programming language [Rust](http://rust-lang.org). This library allows you to draw with vector graphics and then export them as PDFs, SVGs and PDFs. In the future, this library may have a utility for drawing into a GUI window on all operating systems (ie. it would use X11 on linux, Quartz on Mac, Win32 API on Windows, etc).

## Dependencies

You will need the following before you can use this:

* The Rust compiler built and installed on your system.
* The cairo libraries installed on your system. On Unix systems, this should be available in a package on your system's package manager under something like libcairo-dev or you can build it from source via the GNU build system and install it. On Windows, you will need to either build the cairo library as a DLL and then include it with your Rust project's build, or install the GTK+ runtime which will put the cairo DLL in your system's library search path.

## Download

TODO (when cargo-central is available, there will be a quick way to get this installed and use it in your projects)

## Building and Usage

To build this library, make sure you have all the dependencies. Open up your terminal (if you are on Windows, since Rust only supports MinGW building at the moment, this library uses the GNU building system so you need to use the MinGW terminal) simply `cd` into the directory and type:

```sh
make
```

The library file will then be built into lib (if there were no errors during building). You can then bundle this library file inside your Rust project and use Rust's command line option -L to point it to the folder the library is installed in.

To then use the library in your project, you use Rust's `use` statement:

```rust
use std;
use cairo;
```

The library currently requires the std module and for some reason rust refuses to import the std module by default when `use cairo;` is called, so using only `use cairo;` throws compile errors.

## Testing

To run the tests, type:

```sh
make test
```

The tests supplied with libcairo are slowly being ported to these bindings.

## Docs

TODO
