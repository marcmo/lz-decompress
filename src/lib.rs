use libc::{fclose, fopen};
use std::ffi::CString;
use std::os::raw::c_int;
use std::path::Path;

extern "C" {
    fn ffdecompress(infile: *mut libc::c_void, outfile: *mut libc::c_void) -> c_int;
}

/// Decompresses a `.lz` file to the specified output path.
///
/// # Arguments
/// - `input_path`: path to the input `.lz` file.
/// - `output_path`: path to write the decompressed output.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(String)` on failure.
pub fn decompress_file<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), String> {
    unsafe {
        let input = CString::new(input_path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| format!("Invalid input path: {e}"))?;
        let output = CString::new(output_path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| format!("Invalid output path: {e}"))?;
        let read_binary = CString::new("rb").map_err(|e| format!("Invalid mode: {e}"))?;
        let write_binary = CString::new("wb").map_err(|e| format!("Invalid mode: {e}"))?;

        let in_file = fopen(input.as_ptr(), read_binary.as_ptr());
        let out_file = fopen(output.as_ptr(), write_binary.as_ptr());

        if in_file.is_null() || out_file.is_null() {
            return Err("Failed to open input or output file".into());
        }

        let result = ffdecompress(in_file.cast(), out_file.cast());

        fclose(in_file);
        fclose(out_file);

        if result == 0 {
            Ok(())
        } else {
            Err(format!("ffdecompress failed with code {result}"))
        }
    }
}
