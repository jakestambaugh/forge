use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Forgefile {
    /// The command to run to build this project
    build: Option<String>,
    /// The command to run to test this project
    test: Option<String>,
}

pub fn parse_forgefile(path: &Path) -> Forgefile {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let toml_content: Forgefile = toml::from_str(&contents).unwrap();
            toml_content
        }
        Err(_) => panic!(),
    }
}
