use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ranges_containing_other_ranges = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        let mut ranges = line.split(',');

        let left_range = ranges.next().unwrap();

        let mut left_range = left_range.split('-');

        let left_range = left_range.next().unwrap().parse::<u8>().unwrap()
            ..left_range.next().unwrap().parse::<u8>().unwrap();

        let right_range = ranges.next().unwrap();

        let mut right_range = right_range.split('-');

        let right_range = right_range.next().unwrap().parse::<u8>().unwrap()
            ..right_range.next().unwrap().parse::<u8>().unwrap();

        if left_range.start <= right_range.start && left_range.end >= right_range.end {
            ranges_containing_other_ranges += 1;
        } else if left_range.start >= right_range.start && left_range.end <= right_range.end {
            ranges_containing_other_ranges += 1;
        }
    });

    ranges_containing_other_ranges
}

fn part2() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ranges_containing_other_ranges = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        let mut ranges = line.split(',');

        let left_range = ranges.next().unwrap();

        let mut left_range = left_range.split('-');

        let left_range = left_range.next().unwrap().parse::<u8>().unwrap()
            ..=left_range.next().unwrap().parse::<u8>().unwrap();

        let right_range = ranges.next().unwrap();

        let mut right_range = right_range.split('-');

        let right_range = right_range.next().unwrap().parse::<u8>().unwrap()
            ..=right_range.next().unwrap().parse::<u8>().unwrap();

        for item in left_range {
            if right_range.contains(&item) {
                ranges_containing_other_ranges += 1;
                break;
            }
        }
    });

    ranges_containing_other_ranges
}
fn main() {
    println!("result_1: {result_1}", result_1 = part1());
    println!("result_2: {result_2}", result_2 = part2());
}
