use std::io;
use std::fs;

fn main() {
    let data = get_data_from_console();
    let mut linedata = data.lines();

    
    let settings = match linedata.next() {
        Some(line) => line,
        None => return,
    };

    /* let mut settings = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut settings).expect("there should be data here"); */

    let mut settings = settings.trim().split(" ");
    let board_size: isize = settings.next().unwrap().parse().expect("there should be things here");
    let start_pos: isize = settings.next().unwrap().parse::<isize>().unwrap() -1;
    let win_value: isize = settings.next().unwrap().parse().expect("there should be things here");

    let board: Vec<&str> = linedata.next().expect("there should be data").split_whitespace().collect();
    /* let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("there should be data here");
    let board: Vec<&str> = buffer.split_whitespace().collect(); */

    let mut pos = start_pos.clone();
    let mut hops = 0;
    let mut visited = Vec::new();
    loop {
        if board[pos as usize] == win_value.to_string() { 
            println!("magic\n{hops}");
            return; //you win,
        }
        else if visited.contains(&pos) {
            println!("cycle\n{hops}");
            return; //going in circles
        }
        else if pos.is_negative() {
            println!("left\n{hops}");
            return // you lose
        }
        else if pos >= board_size {
            println!("right\n{hops}");
            return // you lose
        }
        visited.push(pos);
        pos += board[pos as usize].parse::<isize>().unwrap();
        hops += 1;
    }
}

fn file_to_string(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("should have read the file");
    content
}

fn get_data_from_console() -> String {
    let mut fline = String::new();
    let mut sline = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut fline);
    stdin.read_line(&mut sline);
    fline.push_str(&sline);
    fline
}

fn get_data_from_file() -> String {
    file_to_string("inputdata.txt")
}