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

fn get_gamma_epsilon(report: &Vec<Vec<i32>>) -> (u32, u32) {
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

fn convert_to_base_10(base2: &Vec<i32>) -> u32 {
    let len = base2.len();
    let mut base10: u32 = 0;

    for i in 0..len {
        if base2[i] == 1 {
            base10 = base10 + 2u32.pow((len - 1 - i) as u32);
        }
    }
    base10
}

fn life_support_rating(
    filter: Vec<&Vec<i32>>,
    index: usize,
    select_most_common: bool,
) -> &Vec<i32> {
    let len = filter.len();

    if len == 1 {
        return filter[0];
    } else {
        if index > 11 {
            panic!("index overflow....");
        }
        let mut filter_0: Vec<&Vec<i32>> = Vec::new();
        let mut filter_1: Vec<&Vec<i32>> = Vec::new();

        for row in filter.iter() {
            match row[index] {
                0 => filter_0.push(row),
                _ => filter_1.push(row),
            };
        }

        let (most_common, least_common) = {
            if filter_1.len() >= filter_0.len() {
                (filter_1, filter_0)
            } else {
                (filter_0, filter_1)
            }
        };

        match select_most_common {
            true => life_support_rating(most_common, index + 1, select_most_common),
            false => life_support_rating(least_common, index + 1, select_most_common),
        }
    }
}

fn main() {
    let mut report = vec![vec![0; 12]; 1000];
    read_report(&mut report);

    let (gamma, epsilon) = get_gamma_epsilon(&report);

    println!(
        "gamma = {}, epsilon = {}, product = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let mut filter: Vec<&Vec<i32>> = Vec::new();
    for i in 0..1000 {
        filter.push(&report[i]);
    }

    let o2 = convert_to_base_10(life_support_rating(filter.clone(), 0, true));
    let co2 = convert_to_base_10(life_support_rating(filter, 0, false));
    println!("o2 = {}, co2 = {}, product = {}", o2, co2, o2 * co2);
}
