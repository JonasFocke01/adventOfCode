// use std::env
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> i32 {    
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut dept_increases: i32 = -1;
    let mut prev_line: i32 = -1;

    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse().unwrap();
        if line > prev_line {
            dept_increases += 1;
        }
        prev_line = line;
    }
    dept_increases
}

fn part2() -> i32 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<i32> = vec![];
    
    let mut dept_increases: i32 = -1;
    let mut new_reading: i32;
    let mut prev_reading: i32 = 0;
    
    for line in reader.lines() {
        let line = line.unwrap().parse().unwrap();
        input_vec.push(line);
    }

    for (i, _reading) in input_vec.iter().enumerate() {
    if i > input_vec.len() -3 {
        continue;
        } else {
            new_reading = input_vec[i] + input_vec[i + 1] + input_vec[i + 2];
            if new_reading > prev_reading {
                dept_increases += 1;
            }
            prev_reading = new_reading;
        }
    }
    dept_increases
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}