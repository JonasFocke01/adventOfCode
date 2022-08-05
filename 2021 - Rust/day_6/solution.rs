use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

const PRODUCTION: bool = true;

fn part1() -> i64 {
    let mut dataset = read_dataset(false);
    let mut temp_dataset: Vec<(u8, i64)> = vec!();
    let mut amount: i64;

    for _ in 0..80 {
        amount = 0;
        for e in dataset.iter() {
            if e.0 == 0 {
                temp_dataset.push((6, e.1));
                amount += e.1;
                // temp_dataset.push((8, 1));
            } else {
                temp_dataset.push((e.0 - 1, e.1));
            }
        }
        if amount > 0 {
            temp_dataset.push((8, amount));
        }
        dataset = temp_dataset;
        temp_dataset = vec!();
    }

    let mut result: i64 = 0;
    for x in dataset.iter() {
        result += x.1;
    }

    result
}

fn part2() -> i64 {
    let mut dataset = read_dataset(false);
    let mut temp_dataset: Vec<(u8, i64)> = vec!();
    let mut amount: i64;

    for _ in 0..256 {
        amount = 0;
        for e in dataset.iter() {
            if e.0 == 0 {
                temp_dataset.push((6, e.1));
                amount += e.1;
            } else {
                temp_dataset.push((e.0 - 1, e.1));
            }
        }
        if amount > 0 {
            temp_dataset.push((8, amount));
        }
        dataset = temp_dataset;
        temp_dataset = vec!();
    }

    let mut result: i64 = 0;
    for x in dataset.iter() {
        result += x.1;
    }

    result
}

fn read_dataset(_verbose: bool) -> Vec<(u8, i64)> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<(u8, i64)> = vec!();

    for line in reader.lines() {
        let line = line.unwrap();
        for character in line.chars() {
            if character.is_numeric() {
                if _verbose { print!("char: {}\n", character); }
                input_vec.push((character as u8 - 0x30, 1));
            }
        }        
    }
    input_vec
}

fn main() {
    print!("Part 1: {:?}\n", part1());
    print!("Part 2: {:?}\n", part2());
}