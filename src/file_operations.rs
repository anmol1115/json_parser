use std::fs::File;
use std::io::{self, BufReader, BufWriter, Error, ErrorKind, Read, Write};

const CHUNK_SIZE: usize = 256 * 1024;

pub fn process_file(src_path: String, dst_path: String) -> io::Result<()> {
    let inp_file_handle = File::open(src_path)?;
    let mut reader = BufReader::new(inp_file_handle);

    let out_file_handle = File::create(dst_path)?;
    let mut writer = BufWriter::new(out_file_handle);

    let mut buffer = vec![0; CHUNK_SIZE];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        let unprocessed_string = String::from_utf8(buffer[..n].to_vec())
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        let processed_string = process_string(unprocessed_string);

        writer.write_all(processed_string.as_bytes())?;
    }

    Ok(())
}

fn process_string(input: String) -> String {
    input.replace(";", ":")
}
