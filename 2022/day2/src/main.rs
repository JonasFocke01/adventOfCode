use std::fs::File;
use std::io::{BufRead, BufReader};

// A, X = Rock
// B, Y = Paper
// C, Z = Scissors

fn part1() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut our_points = 0;

    reader.lines().for_each(|line| {
        let mut line = line.unwrap();
        if line.is_empty() {return};
        let mut line = line.drain(0..=2);
        let theyrs = line.nth(0).to_owned().unwrap();
        let ours = line.nth(1).to_owned().unwrap();
        our_points += match theyrs {
            'A' => match ours{
                'X' => 3,
                'Y' => 6,
                'Z' => 0,
                _ => panic!("pattern not known")
            },
            'B' => match ours{
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => panic!("pattern not known")
            },
            'C' => match ours{
                'X' => 6,
                'Y' => 0,
                'Z' => 3,
                _ => panic!("pattern not known")
            },

            _ => panic!("pattern not known")
        };
        our_points += match ours {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("pattern not known")

        }
    });
    our_points
}

fn part2() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut our_points = 0;

    reader.lines().for_each(|line| {
        let mut line = line.unwrap();
        if line.is_empty() {return};
        let mut line = line.drain(0..=2);
        let theyrs = line.nth(0).to_owned().unwrap();
        let ours = line.nth(1).to_owned().unwrap();
        our_points += match theyrs {
            'A' => match ours{
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => panic!("pattern not known")
            },
            'B' => match ours{
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => panic!("pattern not known")
            },
            'C' => match ours{
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => panic!("pattern not known")
            },
            _ => panic!("pattern not known")
        };
    });
    our_points
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}
