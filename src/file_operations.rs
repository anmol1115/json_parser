use std::fs::File;
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const CHUNK_SIZE: usize = 256 * 1024;
const WORKER_NUM: usize = 4;

fn process_string(input: String) -> String {
    input.replace(";", ":")
}

pub fn process_file_parallel(src_path: String, dst_path: String) -> io::Result<()> {
    let inp_file_handle = Arc::new(Mutex::new(File::open(src_path)?));

    let out_file_handle = File::create(dst_path)?;
    let mut writer = BufWriter::new(out_file_handle);

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    let chunk_position = Arc::new(Mutex::new(0));

    for _ in 0..WORKER_NUM {
        let tx_clone = Arc::clone(&tx);
        let inp_file_clone = Arc::clone(&inp_file_handle);
        let chunk_pos_clone = Arc::clone(&chunk_position);

        thread::spawn(move || loop {
            let mut pos = chunk_pos_clone.lock().unwrap();
            let current_pos = *pos;
            *pos += CHUNK_SIZE;
            drop(pos);

            let mut inp_file = inp_file_clone.lock().unwrap();
            inp_file.seek(SeekFrom::Start(current_pos as u64)).unwrap();

            let mut buffer = vec![0; CHUNK_SIZE];
            let bytes_read = inp_file.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                break;
            }
            let unprocessed_string = String::from_utf8(buffer[..bytes_read].to_vec()).unwrap();
            let processed_string = process_string(unprocessed_string);

            tx_clone.lock().unwrap().send(processed_string).unwrap();
        });
    }
    drop(tx);

    for received in &rx {
        writer.write_all(received.as_bytes())?;
    }

    Ok(())
}
