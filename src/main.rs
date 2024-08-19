use std::io;
//use std::fs;

fn main() {
    //let data = file_to_string("inputdata.txt");
    //let mut linedata = data.lines();
    
    //let settings = linedata.next().expect("something went wrong");
    let mut settings = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut settings).expect("there should be data here");

    let mut settings = settings.trim().split(" ");
    dbg!(&settings);
    let board_size: i32 = settings.next().unwrap().parse().unwrap();
    let start_pos: i32 = settings.next().unwrap().parse::<i32>().unwrap() -1;
    let win_value: i32 = settings.next().unwrap().parse().unwrap();

    //let board: Vec<&str> = linedata.next().expect("there should be data").split_whitespace().collect();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("there should be data here");
    let board: Vec<&str> = buffer.split_whitespace().collect();

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
        else if pos < 1 || pos > board_size {
            println!("falls\n{hops}");
            return // you lose
        }
        visited.push(pos);
        pos += board[pos as usize].parse::<i32>().unwrap();
        hops += 1;
    }
}

/* fn file_to_string(path: &str) -> String {
    let content: String = fs::read_to_string(path).expect("should have read the file");
    content
} */