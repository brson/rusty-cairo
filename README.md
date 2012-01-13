## Synopsis

This project contains bindings of the [cairo vector graphics](http://cairographics.org) library to the system programming language [Rust](http://rust-lang.org). This library allows you to draw with vector graphics and then export them as bitmaps such as PNGs, or export them as vector graphics such as PDFs, SVGs and PS. In the future, this library may have a utility for drawing into a GUI window on all operating systems (ie. it would use X11 on linux, Quartz on Mac, Win32 API on Windows, etc) for making 2d games or visualizations. The library also includes loading of TTF fonts but can fallback to some built-in fonts via cairo's toy font API.  
  
The following is a very basic example and only touches the power of the cairo library:

```rust
use std;
use cairo;
import core::f64::consts;

fn main() {
	let surface = cairo::mk_image_surface(cairo::FORMAT_RGB24, 100u, 100u);
	let context = cairo::mk_context(surface);
	
	context.set_source_rgb(1.0, 0.0, 0.0); // Change our drawing colour to red
	context.paint(); // Paint the entire surface with red
	context.set_source_rgb(1.0, 1.0, 1.0); // Change our drawing colour to white
	context.arc(50.0, 50.0, 50.0, 0.0, core::f64::consts::pi * 2.0); // "Path" a circle at (50, 50) with a radius of 50, and then make it start at 0 radians and then stretch to 2 radians (full circle/arc)
	context.fill(); // This fills paths in with the current colour, hence it fills a white circle
	
	surface.write_to_file("cairo-example.png"); // Finally, render the vector operations we just did into an PNG file
}
```
It renders a vector graphic into the PNG file "cairo-example.png" which *should* contain a red background with a white circle in the middle with it's circumference touching the sides of the image.

## Dependencies

You will need the following before you can use this:

* The Rust compiler built and installed on your system.
* The cairo libraries installed on your system. On Unix systems, this should be available in a package on your system's package manager under something like libcairo-dev or you can build it from source via the GNU build system and install it. On Windows, you will need to either build the cairo library as a DLL and then include it with your Rust project's build, or install the GTK+ runtime which will put the cairo DLL in your system's library search path.
* FreeType, same deal as above.

## Download

TODO (when cargo-central is available, there will be a quick way to get this installed and use it in your projects)

## Building and Usage

To build this library, make sure you have all the dependencies. Open up your terminal (if you are on Windows, since Rust only supports MinGW building at the moment, this library uses the GNU building system so you need to use the MinGW terminal) simply `cd` into the directory and type:

```sh
make
```

The library file will then be built into the `lib` directory (if there were no errors during building). You can then bundle this library file inside your Rust project and use Rust's command line option -L to point it to the folder the library is installed in.

To then use the library in your project, you use Rust's `use` statement:

```rust
use std;
use cairo;
```

The library currently requires the std module and for some reason rust refuses to import the std module by default when `use cairo;` is called, so using only `use cairo;` without first using the std module throws compile errors.

## Testing

To run the tests, type in the terminal:

```sh
make test
```

The tests supplied with libcairo are slowly being ported to these bindings.

## Docs

TODO
