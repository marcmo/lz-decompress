use lz_decompress::decompress_file;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_decompress_file_matches_reference() {
    let input = test_data("mini.txt.lz");
    let expected = test_data("mini.txt");

    let tmp_dir = tempfile::tempdir().unwrap();
    let output = tmp_dir.path().join("out.txt");

    decompress_file(&input, &output).expect("Decompression failed");

    let output_content = fs::read(&output).expect("Failed to read output");
    let expected_content = fs::read(&expected).expect("Failed to read reference");

    assert_eq!(output_content, expected_content);
}

fn test_data(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(file)
}
