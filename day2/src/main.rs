use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(' ');
        let motion = split.next().unwrap();
        let value = split.next().unwrap().parse::<i32>().unwrap();
        match motion {
            "forward" => {
                pos += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("no motion"),
        }
    }
    println!(
        "pos = {}, depth = {}, product = {}",
        pos,
        depth,
        pos * depth
    );
}
