use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::thread;

const PRODUCTION: bool = true;

fn part1() -> i32 {
    let dataset = read_dataset(false);
    
    let mut return_value: i32 = i32::MAX;

    let mut range: (u16, u16) = (100, 0);

    let mut consumption: Vec<i32> = vec!();
    
    for e in dataset.iter() {
        if *e < range.0 {
            range.0 = *e;
        }
        if *e > range.1 {
            range.1 = *e;
        }
    }

    for (i, _location) in dataset.iter().enumerate() {
        consumption.push(calculate_fuel_consumption(i as u16, &dataset, false));
    }

    for consumption_iter in consumption.iter() {
        if *consumption_iter != 0 && *consumption_iter < return_value {
            return_value = *consumption_iter;
        }
    }
        
    return_value
}



fn part2(multithreadding: bool) -> i32 {
    let dataset = read_dataset(false);
    
    let mut return_value: i32 = i32::MAX;
    
    let mut range: (u16, u16) = (100, 0);
    
    let mut consumption: Vec<i32> = vec!();
    
    for e in dataset.iter() {
        if *e < range.0 {
            range.0 = *e;
        }
        if *e > range.1 {
            range.1 = *e;
        }
    }
    
    if multithreadding {
        let mut handlers = vec!();
        let mut index: i32 = 0;
        while dataset.len() > index as usize {
            let dataclone = dataset.to_vec();
            handlers.push(thread::spawn(move || calculate_fuel_consumption(index as u16, &dataclone, true)));
            index += 1;
        }

        while handlers.len() > 0 {
            let handle = handlers.remove(0);
            consumption.push(handle.join().unwrap());
        }

        for consumption_iter in consumption.iter() {
            if *consumption_iter != 0 && *consumption_iter < return_value {
                return_value = *consumption_iter;
            }
        }
    } else {
        for (i, _location) in dataset.iter().enumerate() {
            consumption.push(calculate_fuel_consumption(i as u16, &dataset, true));
        }

        for consumption_iter in consumption.iter() {
            if *consumption_iter != 0 && *consumption_iter < return_value {
                return_value = *consumption_iter;
            }
        }
    }
    
    return_value
}

fn calculate_fuel_consumption(index: u16, input_vec: &Vec<u16>, part_2: bool) -> i32 {
    let mut sum_consumption: i32 = 0;
    for e in input_vec.iter() {
        if *e > index {
            if part_2 {
                sum_consumption += calculate_fuel_increasing_rec(*e as i32, index as i32);
            } else {
                sum_consumption += (*e - index) as i32;
            }
        } else {
            if part_2 {
                sum_consumption += calculate_fuel_increasing_rec(index as i32, *e as i32);
            } else {
                sum_consumption += (index - *e) as i32;
            }
        }
    }
    sum_consumption
}

fn calculate_fuel_increasing_rec(to_index: i32, current_index: i32) -> i32 {
    if to_index == current_index {
        return (to_index - current_index).abs();
    } else if to_index > current_index {
        return (to_index - current_index).abs() + calculate_fuel_increasing_rec(to_index - 1, current_index);
    } else {
        return (to_index - current_index).abs() + calculate_fuel_increasing_rec(to_index, current_index - 1);
    }
}

fn read_dataset(_verbose: bool) -> Vec<u16> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<u16> = vec!();
    let mut current_number: String = String::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        for character in line.chars() {
            if character.is_numeric() {
                current_number.push(character);
            } else {
                input_vec.push(current_number.parse::<u16>().unwrap());
                current_number = String::new();
            }
        }        
        input_vec.push(current_number.parse::<u16>().unwrap());
    }
    input_vec
}

fn main() {
    let solution_part_1 = part1();
    if PRODUCTION {
        assert_eq!(solution_part_1, 335271);
    } else {
        assert_eq!(solution_part_1, 37);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let mut now = Instant::now();
    let mut solution_part_2 = part2(false);
    if PRODUCTION {
        assert_eq!(solution_part_2, 95851339);
    } else {
        assert_eq!(solution_part_2, 168);
    }
    print!("Elapsed time itterative approach: {:?}\n", now.elapsed());
    now = Instant::now();
    solution_part_2 = part2(true);
    if PRODUCTION {
        assert_eq!(solution_part_2, 95851339);
    } else {
        assert_eq!(solution_part_2, 168);
    }
    print!("Elapsed time threadding approach: {:?}\n", now.elapsed());
    print!("Part 2: {}\n", solution_part_2);
}