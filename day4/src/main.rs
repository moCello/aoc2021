use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Entry {
    Value(u8),
    Marked,
}

#[derive(Debug)]
struct Board {
    bingo: bool,
    grid: Vec<Vec<Entry>>,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        if lines.len() != 5 {
            panic!("Incorrect number of lines {}", lines.len());
        }
        let mut grid: Vec<Vec<Entry>> = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            grid.push(Vec::new());
            let mut row_entries = line.split_whitespace();
            for _ in 0..5 {
                let value = row_entries
                    .next()
                    .expect("can not move to next value")
                    .parse::<u64>()
                    .expect("can not parse value to u8");
                let entry = Entry::Value(value as u8);
                grid[row].push(entry);
            }
        }

        Board { bingo: false, grid }
    }

    // Marks a drawn number in all the boards and checks whether there is a Bingo.
    fn mark(&mut self, number: u8) -> Option<(usize, u8)> {
        for line in self.grid.iter_mut() {
            for entry in line.iter_mut() {
                match *entry {
                    Entry::Value(value) => {
                        if value == number {
                            *entry = Entry::Marked;
                        }
                    }
                    Entry::Marked => {}
                }
            }
        }
        None
    }

    // Returns None for no bingo or Some(board_id, drawn_number) for bingo.
    fn is_bingo(&mut self) -> bool {
        let mut count = 0;
        for line in self.grid.iter() {
            for entry in line.iter() {
                match *entry {
                    Entry::Value(_) => count = 0,
                    Entry::Marked => {
                        count += 1;
                    }
                }
            }
            if count == 5 {
                self.bingo = true;
                return true;
            } else {
                count = 0;
            }
        }
        for col in 0..5 {
            for line in 0..5 {
                match self.grid[line][col] {
                    Entry::Value(_) => count = 0,
                    Entry::Marked => {
                        count += 1;
                    }
                }
            }
            if count == 5 {
                self.bingo = true;
                return true;
            } else {
                count = 0;
            }
        }
        false
    }

    // Calculates the result for the board
    fn get_result(&self, drawn_number: u8) -> u32 {
        let mut sum: u32 = 0;
        for line in self.grid.iter() {
            for entry in line.iter() {
                match *entry {
                    Entry::Value(value) => {
                        sum += value as u32;
                    }
                    Entry::Marked => {}
                }
            }
        }
        println!("sum {}", sum);
        sum * drawn_number as u32
    }
}

struct Game {
    drawn_numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn new() -> Self {
        Game {
            drawn_numbers: Vec::new(),
            boards: Vec::new(),
        }
    }

    fn from_input(&mut self, input: &str) {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let line = lines.next().unwrap().unwrap();

        // read drawn numbers
        let numbers = line.split(',');
        for number in numbers {
            let number = number.parse::<u8>().unwrap();
            self.drawn_numbers.push(number);
        }

        // read boards
        loop {
            let line = lines.next();
            match line {
                None => break,
                Some(line) => {
                    let line = line.unwrap();
                    let mut collector = Vec::new();
                    if line == String::from("") {
                        for _ in 0..5 {
                            let line = lines.next().unwrap().unwrap();
                            collector.push(line);
                        }
                    }
                    self.boards.push(Board::new(collector));
                }
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.from_input("input");
    let mut boards_left = game.boards.len();
    for &drawn in game.drawn_numbers.iter() {
        for board in game.boards.iter_mut() {
            // only mark number on a board that has no bingo yet
            if board.bingo == false {
                board.mark(drawn);
                // check if we got a fresh bingo
                if board.is_bingo() {
                    boards_left -= 1;
                }
            }
            if boards_left == 0 {
                println!(
                    "last board {:?} with result {}",
                    board,
                    board.get_result(drawn)
                );
                return;
            }
        }
    }
}
