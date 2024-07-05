use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};

use json_parser::path_exists;

const CHUNK_SIZE: usize = 256 * 1024;

pub fn process_file(src_path: String, dst_path: String) -> io::Result<()> {
    let file_handle = File::open(src_path)?;
    let mut reader = BufReader::new(file_handle);
    let mut buffer = vec![0; CHUNK_SIZE];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        write_to_file(
            &dst_path,
            process_string(String::from_utf8_lossy(&buffer[..n]).to_string()),
        )?;
    }

    Ok(())
}

fn process_string(input: String) -> String {
    input.replace(";", ":")
}

fn write_to_file(dst_path: &str, file_chunk: String) -> io::Result<()> {
    if !path_exists(dst_path) {
        let _ = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dst_path);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(dst_path)
        .unwrap();
    writeln!(file, "{}", file_chunk)?;

    Ok(())
}
