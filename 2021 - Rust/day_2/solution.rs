use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> i32 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<(String, i32)> = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input_vec.push((String::new(), 0));
        for character in line.chars() {
            if character != ' ' {
                if character.is_numeric() == true {
                    input_vec[i].1 = character as i32 - 0x30;
                } else {
                    input_vec[i].0.push(character);
                }
            }
        }
    }
    
    //from here on, input_vec is a Vector with the input

    let mut forward: i32 = 0;
    let mut vertical: i32 = 0;

    for (_i, instruction) in input_vec.iter().enumerate() {
        if instruction.0 == String::from("forward") {
            forward += instruction.1;
        } else if instruction.0 == String::from("up") {
            vertical -= instruction.1
        } else if instruction.0 == String::from("down") {
            vertical += instruction.1
        }
    }

    forward * vertical
}

fn part2() -> i32 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<(String, i32)> = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input_vec.push((String::new(), 0));
        for character in line.chars() {
            if character != ' ' {
                if character.is_numeric() == true {
                    input_vec[i].1 = character as i32 - 0x30;
                } else {
                    input_vec[i].0.push(character);
                }
            }
        }
    }
    
    //from here on, input_vec is a Vector with the input

    let mut forward: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for (_i, instruction) in input_vec.iter().enumerate() {
        if instruction.0 == String::from("forward") {
            forward += instruction.1;
            depth += instruction.1 * aim;
        } else if instruction.0 == String::from("up") {
            aim -= instruction.1
        } else if instruction.0 == String::from("down") {
            aim += instruction.1
        }
    }
    forward * depth
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {:?}\n", part2());
}