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
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<String> = vec![];
    
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input_vec.push(line);
    }
 
    //from here on, input_vec is a Vector with the input

    let _co2: i32;

    i32::from_str_radix(&part2_filter_report_by(&input_vec, 0, true)[0], 2).unwrap() * i32::from_str_radix(&part2_filter_report_by(&input_vec, 0, false)[0], 2).unwrap()
}

fn part2_filter_report_by(dataset: &Vec<String>, position: u8, find_oxygen: bool) -> Vec<String> {
    let mut crafted_dataset: Vec<String> = vec![];
    let mut counter1: i32 = 0;
    let mut counter0: i32 = 0;

    for e in dataset.iter() {
        if e.chars().nth(position.into()) == Some('1') {
            counter1 += 1;
        } else {
            counter0 += 1;
        }
    }

    for (i, el) in dataset.iter().enumerate() {
        if counter1 > counter0 {
            if find_oxygen == true {
                if el.chars().nth(position.into()) == Some('1') {
                    crafted_dataset.push(String::from(el));
                }
            } else {
                if el.chars().nth(position.into()) == Some('0') {
                    crafted_dataset.push(String::from(el));
                }
            }
        } else if counter1 < counter0 {
            if find_oxygen == true {
                if el.chars().nth(position.into()) == Some('0') {
                    crafted_dataset.push(String::from(el));
                }
            } else {
                if el.chars().nth(position.into()) == Some('1') {
                    crafted_dataset.push(String::from(el));
                }
            }
        } else {
            if find_oxygen == true {
                if el.chars().nth(position.into()) == Some('1') {
                    crafted_dataset.push(String::from(el));
                }
            } else {
                if el.chars().nth(position.into()) == Some('0') {
                    crafted_dataset.push(String::from(el));
                }
            }
        }
    }

    // print!("crafted_dataset: {:?} | position: {} | length: {}\n", crafted_dataset, position, crafted_dataset.len());

    if crafted_dataset.len() > 1 && dataset[0].len() > position.into() {
        part2_filter_report_by(&crafted_dataset, position + 1, find_oxygen)
    } else {
        print!("recursive done with result: {:?}\n", crafted_dataset);
        crafted_dataset
    }
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}