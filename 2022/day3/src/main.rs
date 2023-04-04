use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut left_compartment: Vec<char> = vec![];
    let mut right_compartment: Vec<char> = vec![];
    let mut common_items: Vec<char> = vec![];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        (0..(line.len() / 2)).for_each(|char_i| {
            left_compartment.push(line.chars().nth(char_i).unwrap());
        });

        ((line.len() / 2)..line.len()).for_each(|char_i| {
            right_compartment.push(line.chars().nth(char_i).unwrap());
        });

        if let Some(c) = left_compartment
            .iter()
            .find(|item| right_compartment.contains(item))
        {
            common_items.push(*c);
        };

        left_compartment = vec![];
        right_compartment = vec![];
    });

    common_items
        .iter()
        .map(|item| {
            if item.is_lowercase() {
                return ((*item as u8) - 96) as u64;
            } else {
                return ((*item as u8) - 64 + 26) as u64;
            }
        })
        .sum()
}

fn part2() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut groups = (vec![], vec![], vec![]);
    let mut current_group = 1;
    let mut result = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };
        let line = line.chars().map(|item| {
            if item.is_lowercase() {
                return ((item as u8) - 96) as u64;
            } else {
                return ((item as u8) - 64 + 26) as u64;
            }
        });
        match current_group {
            1 => groups.0 = line.collect::<Vec<u64>>().to_vec(),
            2 => groups.1 = line.collect::<Vec<u64>>().to_vec(),
            3 => groups.2 = line.collect::<Vec<u64>>().to_vec(),
            _ => panic!(),
        }

        if current_group == 3 {
            if let Some(c) = groups
                .0
                .iter()
                .find(|item| return groups.1.contains(item) && groups.2.contains(item))
            {
                result += c
            };
            current_group = 1;
        } else {
            current_group += 1;
        }
    });
    result
}
fn main() {
    println!("{result_1}", result_1 = part1());
    println!("{result_2}", result_2 = part2());
}
