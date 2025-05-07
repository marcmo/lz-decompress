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

## Windows Notes

This crate supports building on Windows with both `x86_64` and `aarch64` MSVC targets.

### Prerequisites

1. **Install MSVC Build Tools**, including required architectures:
   - Open the **Visual Studio Installer**
   - Select **"Desktop development with C++"**
   - In the "Individual components" tab, ensure the following are checked:
     - **MSVC v14.x x64/x86 build tools**
     - **MSVC v14.x ARM64 build tools** (if targeting ARM64)
     - **Windows 10/11 SDK**
     - Optional but useful: **CMake**, **Ninja**

2. **Install Rust targets** as needed:

   ```sh
   rustup target add x86_64-pc-windows-msvc
   rustup target add aarch64-pc-windows-msvc
   ```

2. **Rust target is installed**:

   ```sh
   rustup target add aarch64-pc-windows-msvc
   ```

3. **Building**

### For x86_64

```sh
cargo build --target x86_64-pc-windows-msvc
```

### For aarch64 (ARM64)

```sh
cargo build --target aarch64-pc-windows-msvc
```

## Notes

- The C wrapper is compatible with MSVC and does not require POSIX headers like unistd.h.
- When building C code via the cc crate, make sure environment variables for the compiler are set correctly. You can ensure this by:
- Launching from the “x64 Native Tools Command Prompt for VS 2022” (for x86_64), or
- Running vcvars64.bat / vcvarsarm64.bat manually depending on your target.

## License

This crate is distributed under the terms of the 2-clause BSD license, the same as the upstream lzlib C library.

See LICENSE.txt for full text.
Original C code copyright © Antonio Diaz Diaz.
