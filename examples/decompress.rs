fn main() {
    match lz_decompress::decompress_file("examples/test.txt.lz", "examples/test.out") {
        Ok(()) => println!("Decompression successful."),
        Err(e) => eprintln!("Decompression failed: {e}"),
    }
}
