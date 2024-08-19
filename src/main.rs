//use std::io;
use std::fs;

fn main() {
    let data = fileToString("inputdata.txt");
    let mut linedata = data.lines();
    
    let settings = linedata.next().expect("something went wrong");
    
    let mut settings = settings.split(" ");
    let board_size: i32 = settings.next().unwrap().parse().unwrap();
    let start_pos: i32 = settings.next().unwrap().parse().unwrap();
    let win_value: i32 = settings.next().unwrap().parse().unwrap();

    let board = linedata.next().expect("there should be data");

    println!("{start_pos}");
}

fn fileToString(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("should have read the file");
    content
}