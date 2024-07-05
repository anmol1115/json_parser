use std::env;

mod file_operations;
use file_operations::process_file;
use json_parser::validate_args;

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_args(args) {
        Ok((src, dst)) => match process_file(src, dst) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        Err(e) => panic!("{}", e),
    }
}
