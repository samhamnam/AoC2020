use std::fs::File;
use std::io::prelude::*;

pub fn load_input(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open input!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read!");

    content
}
