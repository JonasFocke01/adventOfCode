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

        // print!("line: {:?}\n", input_vec[i]);
    }
    
    //from here on, input_vec is a Vector with the input

    let mut solution: i32 = 0;

    let mut forward: i32 = 0;
    let mut vertical: i32 = 0;

    for (i, instruction) in input_vec.iter().enumerate() {

        solution = forward + vertical;

        if instruction.0 == String::from("forward") {
            forward += instruction.1;
        } else if instruction.0 == String::from("up") {
            vertical -= instruction.1
        } else if instruction.0 == String::from("down") {
            vertical += instruction.1
        }
        // print!("instruction: {:?}, forward: {}, vertical: {}\n", instruction, forward, vertical);
    }

    solution = forward * vertical;

    solution    
}

fn part2() {
    // let filename = "input.txt";
    // let file = File::open(filename).unwrap();
    // let reader = BufReader::new(file);
    // let mut input_vec: Vec<i32> = vec![];
    // let mut solution: i32 = 0;
   
    // for line in reader.lines() {
    //     let line = line.unwrap().parse().unwrap();
    //     input_vec.push(line);
    // }

    // //from here on, input_vec is a Vector with the input
    
    // for (_i, _reading) in input_vec.iter().enumerate() {
        
    // }
    // solution    
}

fn main() {

    print!("Part 1: {}\n", part1());
    print!("Part 2: {:?}\n", part2());
    print!("u32 max {}\n", u32::MAX);
    print!("i32 max {}\n", i32::MAX);
    print!("u16 max {}\n", u16::MAX);
}