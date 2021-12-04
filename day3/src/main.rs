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

fn make_new_selector(
    report: &Vec<Vec<i32>>,
    selector: &Vec<usize>,
    new_selector: &mut Vec<usize>,
    index: usize,
    bit: i32,
) {
    if index > 11 {
        panic!("buffer overflow when making new selector");
    }

    for row in selector.iter() {
        if report[*row][index] == bit {
            new_selector.push(*row);
        }
    }
}

fn life_support_rating<'a>(
    report: &'a Vec<Vec<i32>>,
    selector: Vec<usize>,
    index: usize,
    most_common: bool,
) -> &'a Vec<i32> {
    let len = selector.len();

    if len == 1 {
        return &report[selector[0]];
    } else {
        let mut new_selector: Vec<usize> = Vec::new();
        let mut count = 0;

        for row in selector.iter() {
            count += report[*row][index];
        }

        let (most_common_bit, least_common_bit) = {
            if count * 2 >= len as i32 {
                (1, 0)
            } else {
                (0, 1)
            }
        };

        match most_common {
            true => make_new_selector(report, &selector, &mut new_selector, index, most_common_bit),
            false => make_new_selector(
                report,
                &selector,
                &mut new_selector,
                index,
                least_common_bit,
            ),
        };

        life_support_rating(report, new_selector, index + 1, most_common)
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

    let mut selector: Vec<usize> = Vec::new();
    for i in 0..1000 {
        selector.push(i);
    }

    let o2 = convert_to_base_10(life_support_rating(&report, selector.clone(), 0, true));
    let co2 = convert_to_base_10(life_support_rating(&report, selector, 0, false));
    println!("o2 = {}, co2 = {}, product = {}", o2, co2, o2 * co2);
}
