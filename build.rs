use std::path::Path;

fn main() {
    let lz = "lzlib-src";
    let lzlib = Path::new(&lz);

    cc::Build::new()
        .file("wrapper/lzlib_wrapper.c")
        .file(lzlib.join("lzlib.c"))
        .include(lz)
        .include("wrapper")
        .flag_if_supported("-std=c99")
        .compile("lzlib_wrapper");

    println!("cargo:rerun-if-changed=wrapper/lzlib_wrapper.c");
    println!("cargo:rerun-if-changed=lzlib-src/decoder.c");
}
