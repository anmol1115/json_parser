use std::path::Path;

pub fn validate_args(args: Vec<String>) -> Result<(String, String), String> {
    if args.len() != 3 {
        return Err("Invalid number of arguments passed".to_string())
    }

    let src_path = &args[1];
    let dst_path = &args[2];

    if !path_exists(&src_path) {
        return Err("Input file not found".to_string())
    }

    Ok((src_path.to_string(), dst_path.to_string()))
}

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
