use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let mut msd: char = ' ';
        let mut lsd: char = ' ';
        for line_char in l.chars() {
            if line_char.is_digit(10) && lsd != '0' {
                lsd = line_char;
            }
        }
        for line_char in l.chars().rev() {
            if line_char.is_digit(10) && msd != '0' {
                msd = line_char;
            }
        }

        let addable_number = (lsd as u64 - 48) + ((msd as u64 - 48) * 10);
        result += addable_number;
    });
    result.into()
}

fn part2() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let mut msd: u64 = 0;
        let mut lsd: u64 = 0;
        let mut chars: Vec<char> = vec![];
        for line_char in l.chars() {
            if line_char.is_digit(10) && lsd == 0 {
                lsd = (line_char as u64 - 48) * 10;
            }
            chars.push(line_char);
            if lsd == 0 {
                if let Some(new_lsd) = part2_help(chars.iter().collect()) {
                    lsd = new_lsd * 10;
                }
            }
        }
        chars = vec![];
        for line_char in l.chars().rev() {
            if line_char.is_digit(10) && msd == 0 {
                msd = line_char as u64 - 48;
            }
            chars.push(line_char);
            if msd == 0 {
                if let Some(new_msd) = part2_help(chars.iter().rev().collect()) {
                    msd = new_msd;
                }
            }
        }

        let addable_number = lsd + msd;
        result += addable_number;
    });
    result.into()
}

fn part2_help(chars: Vec<&char>) -> Option<u64> {
    let chars = chars.iter().cloned().collect::<String>();
    if chars.contains("one") {
        Some(1)
    } else if chars.contains("two") {
        Some(2)
    } else if chars.contains("three") {
        Some(3)
    } else if chars.contains("four") {
        Some(4)
    } else if chars.contains("five") {
        Some(5)
    } else if chars.contains("six") {
        Some(6)
    } else if chars.contains("seven") {
        Some(7)
    } else if chars.contains("eight") {
        Some(8)
    } else if chars.contains("nine") {
        Some(9)
    } else {
        None
    }
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}
