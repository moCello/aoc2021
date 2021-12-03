use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_report(report: &mut Vec<Vec<i32>>) {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    for (line_no, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line_chars = line.chars();
        for (index, character) in line_chars.into_iter().enumerate() {
            if character == '1' {
                report[line_no][index] = 1;
            }
        }
    }
}

fn get_rates(report: &Vec<Vec<i32>>) -> (u32, u32) {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for col in 0..12 {
        let mut count = 0;
        for row in 0..1000 {
            count += report[row][col];
        }
        if count == 500 {
            panic!("equal amount of 0 and 1 bits");
        }
        match count > 500 {
            true => gamma = gamma + 2u32.pow(11 - col as u32),
            false => epsilon = epsilon + 2u32.pow(11 - col as u32),
        }
    }
    (gamma, epsilon)
}

fn main() {
    let mut report = vec![vec![0; 12]; 1000];
    read_report(&mut report);
    let (gamma, epsilon) = get_rates(&report);
    println!(
        "gamma = {}, epsilon = {}, product = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
