use std::env;
use std::fs::File;
use std::io::{self, Read};

pub fn read_file(path: &str) -> io::Result<String> {
    let exe_path = env::current_dir()?;
    let static_path = exe_path.join("static").join(path);
    let mut file = File::open(static_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
