use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(clippy::all)]
fn part1() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    const MAX_OF_POSSIBLE_GAMES: (u8, u8, u8) = (12, 13, 14);

    let mut result = 0;

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let mut game_max = (0, 0, 0);

        let mut game_rest = l.split(":");
        let game_str = game_rest.next().unwrap();
        let game_num_str = game_str.replace("Game ", "");
        let game_num: u64 = game_num_str.parse().unwrap();
        let game_peaks = game_rest.next().unwrap().split(";");

        for peak in game_peaks {
            for color in peak.split(",") {
                let mut color = color.replace(" ", "");
                if color.contains("red") {
                    color = color.replace("red", "");
                    let red: u8 = color.parse().unwrap();
                    if red > game_max.0 {
                        game_max.0 = red;
                    }
                }
                if color.contains("green") {
                    color = color.replace("green", "");
                    let green: u8 = color.parse().unwrap();
                    if green > game_max.1 {
                        game_max.1 = green;
                    }
                }
                if color.contains("blue") {
                    color = color.replace("blue", "");
                    let blue: u8 = color.parse().unwrap();
                    if blue > game_max.2 {
                        game_max.2 = blue;
                    }
                }
            }
        }
        if game_max.0 <= MAX_OF_POSSIBLE_GAMES.0
            && game_max.1 <= MAX_OF_POSSIBLE_GAMES.1
            && game_max.2 <= MAX_OF_POSSIBLE_GAMES.2
        {
            result += game_num;
        }
    });
    result
}

#[allow(clippy::all)]
fn part2() -> u64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let mut game_max = (0, 0, 0);

        let mut game_rest = l.split(":");
        let game_peaks = game_rest.next().unwrap().split(";");

        for peak in game_peaks {
            for color in peak.split(",") {
                let mut color = color.replace(" ", "");
                if color.contains("red") {
                    color = color.replace("red", "");
                    let red: u64 = color.parse().unwrap();
                    if red > game_max.0 {
                        game_max.0 = red;
                    }
                }
                if color.contains("green") {
                    color = color.replace("green", "");
                    let green: u64 = color.parse().unwrap();
                    if green > game_max.1 {
                        game_max.1 = green;
                    }
                }
                if color.contains("blue") {
                    color = color.replace("blue", "");
                    let blue: u64 = color.parse().unwrap();
                    if blue > game_max.2 {
                        game_max.2 = blue;
                    }
                }
            }
        }
        result += game_max.0 * game_max.1 * game_max.2;
    });
    result
}

#[allow(clippy::all)]
fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}
