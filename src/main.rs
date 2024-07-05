use std::env;

mod file_operations;
use file_operations::process_file;
use json_parser::validate_args;

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_args(args) {
        Ok((src, dst)) => process_file(src, dst),
        Err(e) => panic!("{}", e),
    }
}
