//use std::io;
use std::fs;

fn main() {
    let data = file_to_string("inputdata.txt");
    let mut linedata = data.lines();
    
    let settings = linedata.next().expect("something went wrong");
    
    let mut settings = settings.split(" ");
    let board_size: usize = settings.next().unwrap().parse().unwrap();
    let start_pos: i32 = settings.next().unwrap().parse().unwrap();
    let win_value: i32 = settings.next().unwrap().parse().unwrap();

    let board: Vec<&str> = linedata.next().expect("there should be data").split_whitespace().collect();
    let mut pos = start_pos.clone();
    let mut hops = 0;
    loop {
        if board[pos as usize] == win_value.to_string() { 
            println!("magic\n{hops}");
            return; //you win,
        }
        else if pos == start_pos && hops != 0 {
            println!("cycle\n{hops}");
            return; //going in circles
        }
        else if board[pos as usize].parse::<usize>().unwrap() < 1 || board[pos as usize].parse::<usize>().unwrap() > board_size {
            println!("falls\n{hops}");
            return // you lose
        }
        pos = board[pos as usize].parse::<i32>().unwrap();
        hops += 1;
    }
    

    //println!("{}",board[start_pos]);
}

fn file_to_string(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("should have read the file");
    content
}