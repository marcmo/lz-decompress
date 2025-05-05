# lz-decompress

Rust bindings for the [lzlib](https://www.nongnu.org/lzip/lzlib.html) compression library — providing decompression support for `.lz` files.

This crate offers a safe Rust interface to a subset of `lzlib`, exposing a function to decompress `.lz` files.

## Features

- Decompress `.lz` files using `lzlib`
- Simple, safe Rust API
- tested with integration tests

## Usage

``` Rust
use std::path::Path;
use lzlib_rs::decompress_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Path::new("test.txt.lz");
    let output = Path::new("test.txt");

    decompress_file(input, output)?;

    println!("Decompressed to {}", output.display());
    Ok(())
}
```

## License

This crate is distributed under the terms of the 2-clause BSD license, the same as the upstream lzlib C library.

See LICENSE.txt for full text.
Original C code copyright © Antonio Diaz Diaz.
