use std::fs::File;
use std::io::{BufRead, BufReader};

const PRODUCTION: bool = true;
const RANGE_X: u16 = 99;
const RANGE_Y: u16 = 100;
// const RANGE_X: u16 = 9;
// const RANGE_Y: u16 = 5;

fn part1() -> u16 {
    let dataset = read_dataset(false);
    let mut return_value: u16 = 0 ;

    for (i, row) in dataset.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            return_value += check_if_low_point((i as u16, j as u16), &dataset);
        }
    }

    return_value
}



fn part2() -> i32 {
    let dataset = read_dataset(false);
    
    let return_value: i32 = -1;

    return_value
}

fn check_if_low_point(point: (u16, u16), input: &Vec<Vec<u8>>) -> u16 {
    let value = input[point.0 as usize][point.1 as usize];
    let mut found = 0;
    if point.0 > 0 {
        if input[point.0 as usize - 1][point.1 as usize] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.1 > 0 {
        if input[point.0 as usize][point.1 as usize - 1] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.0 < RANGE_Y - 1 {
        if input[point.0 as usize + 1][point.1 as usize] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.1 < RANGE_X {
        if input[point.0 as usize][point.1 as usize + 1] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if found == 4 {
        return input[point.0 as usize][point.1 as usize] as u16 + 1;
    } else {
        return 0;
    }
}

fn read_dataset(_verbose: bool) -> Vec<Vec<u8>> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<u8>> = vec!();
    
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input.push(vec!());
        for character in line.chars() {
            input[i].push(character as u8 - 0x30);
        }        
        if _verbose { print!("linebreak\n"); }
    }
    input
}

fn main() {
    let solution_part_1 = part1();
    if PRODUCTION {
        assert_eq!(solution_part_1, 528);
    } else {
        assert_eq!(solution_part_1, 15);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let solution_part_2 = part2();
    if PRODUCTION {
        assert_eq!(solution_part_2, -1);
    } else {
        assert_eq!(solution_part_2, -1);
    }
    print!("Part 2: {}\n", solution_part_2);
}