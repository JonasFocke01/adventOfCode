use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> String {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut parsing_crates = true;

    let mut stacks: Vec<Vec<u8>> = vec![vec![]; 9];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        if parsing_crates {
            let mut line = line.bytes();
            if line.nth(0).unwrap() == 32 {
                parsing_crates = false;
                stacks = stacks
                    .iter_mut()
                    .map(|inner_stack| {
                        inner_stack.reverse();
                        inner_stack.retain(|item| *item != 32);
                        return inner_stack.to_vec();
                    })
                    .collect();
                return;
            }
            stacks[0].push(line.nth(0).unwrap());
            (1..9).for_each(|i| {
                stacks[i].push(line.nth(3).unwrap());
            });
        } else {
            let mut line = line.split(' ');
            let move_amount = line.nth(1).unwrap().parse::<u8>().unwrap();
            let source = line.nth(1).unwrap().parse::<u8>().unwrap();
            let dest = line.nth(1).unwrap().parse::<u8>().unwrap();
            // println!("move {} from {} to {}", move_amount, source, dest);
            (0..move_amount).for_each(|_| {
                let source = stacks[source as usize - 1].pop().unwrap();
                stacks[dest as usize - 1].push(source);
            })
        }
    });

    let mut result: String = String::new();

    stacks
        .iter_mut()
        .for_each(|inner_vec| result.push(inner_vec.pop().unwrap().into()));

    result
}

fn part2() -> String {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut parsing_crates = true;

    let mut stacks: Vec<Vec<u8>> = vec![vec![]; 9];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        if parsing_crates {
            let mut line = line.bytes();
            if line.nth(0).unwrap() == 32 {
                parsing_crates = false;
                stacks = stacks
                    .iter_mut()
                    .map(|inner_stack| {
                        inner_stack.reverse();
                        inner_stack.retain(|item| *item != 32);
                        return inner_stack.to_vec();
                    })
                    .collect();
                return;
            }
            stacks[0].push(line.nth(0).unwrap());
            (1..9).for_each(|i| {
                stacks[i].push(line.nth(3).unwrap());
            });
        } else {
            let mut line = line.split(' ');
            let move_amount = line.nth(1).unwrap().parse::<u8>().unwrap();
            let source = line.nth(1).unwrap().parse::<u8>().unwrap();
            let dest = line.nth(1).unwrap().parse::<u8>().unwrap();
            let mut crates: Vec<u8> = vec![];

            (0..move_amount).for_each(|_| {
                crates.push(stacks[source as usize - 1].pop().unwrap());
            });
            crates.reverse();
            stacks[dest as usize - 1].append(&mut crates);
        }
    });

    let mut result: String = String::new();

    stacks
        .iter_mut()
        .for_each(|inner_vec| result.push(inner_vec.pop().unwrap().into()));

    result
}

fn main() {
    println!("result_1: {result_1}", result_1 = part1());
    println!("result_2: {result_2}", result_2 = part2());
}
