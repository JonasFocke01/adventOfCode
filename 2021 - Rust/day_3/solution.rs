use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> i32 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<String> = vec![];

    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input_vec.push(line);
    }
    
    print!("Inputvector: {:?}\n", input_vec);
    //from here on, input_vec is a Vector with the input

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for digit in 0..12 {
        let mut count1: i32 = 0;
        let mut count0: i32 = 0;
        for input in input_vec.iter() {
            if input.chars().nth(digit) == Some('1') {
                count1 += 1;
            } else {
                count0 += 1;
            }
        }
        if count1 > count0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

fn part2() -> i32 {
    -1
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}