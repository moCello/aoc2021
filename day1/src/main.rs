use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut new_sum = 0;
    let mut hight_off_1 = 0;
    let mut hight_off_2 = 0;
    let mut hight_off_3 = 0;
    let mut counter = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let new_hight = line.parse::<i32>().unwrap();
        match index {
            0 => {
                hight_off_3 = new_hight;
                sum += new_hight;
                new_sum += new_hight;
            }
            1 => {
                hight_off_2 = new_hight;
                sum += new_hight;
                new_sum += new_hight;
            }
            2 => {
                hight_off_1 = new_hight;
                sum += new_hight;
                new_sum += new_hight;
            }
            _ => {
                sum = new_sum;
                new_sum = new_sum + new_hight - hight_off_3;
                hight_off_3 = hight_off_2;
                hight_off_2 = hight_off_1;
                hight_off_1 = new_hight;
            }
        }

        if new_sum > sum {
            counter += 1;
        }
        sum = new_sum;
    }
    println!("{} increases", counter);
}
