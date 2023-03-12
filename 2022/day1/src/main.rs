// use std::env
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve() -> Vec<u64> {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elfs = vec!();
    
    reader.lines().for_each(|line| {
        let l = line.unwrap();
            if l.is_empty() {
                elfs.push(0)
            } else {
                let mut elf = elfs.pop().unwrap_or(0);
                elf += l.parse::<u64>().unwrap();
                elfs.push(elf);
            }    
        
    });
    elfs.sort();
    elfs
}

fn part1() -> u64 {
    solve().pop().unwrap()
}

fn part2() -> u64 {
    let mut elfs = solve();
    let mut result = elfs.pop().unwrap();
    result += elfs.pop().unwrap();
    result += elfs.pop().unwrap();
    result
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}
