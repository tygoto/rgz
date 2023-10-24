use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn get_proto_file_paths(dir_path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut file_paths = Vec::new();
    let ext = "proto";
    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        if let Some(file_ext) = file_path.extension() {
            if file_ext == ext {
                let mut p = PathBuf::from(dir_path);
                p.push(entry.file_name());
                file_paths.push(p);
            }
        }
    }
    Ok(file_paths)
}

fn main() -> Result<(), Box<dyn Error>> {
    let gz_msgs_file_paths = get_proto_file_paths("gz-msgs/proto/gz/msgs")?;

    prost_build::Config::new()
        .message_attribute(".", "#[derive(::rgz_derive::GzMessage)]")
        .out_dir("src")
        .compile_protos(&gz_msgs_file_paths, &[&"gz-msgs/proto"])?;
    Ok(())
}
