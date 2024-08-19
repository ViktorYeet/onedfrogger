use std::io;
use std::fs;

fn main() {
    let data = fileToString("inputdata.txt");

    data.

    println!("{data}");
}

fn fileToString(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("should have read the file");
    content
}