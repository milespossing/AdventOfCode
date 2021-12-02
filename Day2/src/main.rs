use std::env;
use std::fs;

#[derive(Debug)]
struct sub{
    x:u32,
    y:u32,
}

enum sub_move {
    forward(u32),
    up(u32),
    down(u32),
}

fn process_move(line: &str) -> sub_move {
    let args = line.split_whitespace().collect::<Vec<&str>>();
    let magnitude = args[1].parse::<u32>().unwrap();
    match args[0] {
        "forward" => sub_move::forward(magnitude),
        "up" => sub_move::up(magnitude),
        "down" => sub_move::down(magnitude),
        _ => unreachable!(),
    }
}

fn main() {
    let mut sub = sub{x: 0, y: 0};
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Could not read");
    let lines = contents.lines();
    println!("{}", filename);
    for line in lines {
        match process_move(line){
            sub_move::forward(magnitude) => sub.x += magnitude,
            sub_move::down(magnitude) => sub.y += magnitude,
            sub_move::up(magnitude) => sub.y -= magnitude,
        }
    }
    println!("{:?}", sub);
    println!("{}", sub.x * sub.y);
    // println!("{:?}", lines);
}
