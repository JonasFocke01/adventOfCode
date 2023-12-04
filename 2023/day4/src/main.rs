use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    our_numbers: Vec<usize>,
    part_one_result: usize,
    winns: usize,
    copies: usize,
}
fn parse_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let mut winning_numbers: Vec<usize> = vec![];
        let mut our_numbers: Vec<usize> = vec![];
        let mut numbers = l.split(": ");
        numbers.next();
        let mut numbers_string = numbers.next().unwrap().split(" | ");
        let mut winning_numbers_string = numbers_string.next().unwrap();
        let mut our_numbers_string = numbers_string.next().unwrap();
        for number_string in winning_numbers_string.split(" ") {
            if number_string != "" {
                winning_numbers.push(number_string.parse().unwrap());
            }
        }
        for number_string in our_numbers_string.split(" ") {
            if number_string != "" {
                our_numbers.push(number_string.parse().unwrap());
            }
        }

        let mut card_result = 0;
        let mut winns = 0;
        for winning_number in &winning_numbers {
            if our_numbers.contains(&winning_number) {
                winns += 1;
                if card_result == 0 {
                    card_result += 1;
                } else {
                    card_result *= 2;
                }
            }
        }

        cards.push(Card {
            winning_numbers,
            our_numbers,
            part_one_result: card_result,
            winns: winns,
            copies: 0,
        });
    });
    cards
}
fn part1() -> usize {
    let mut result = 0;

    let cards = parse_cards();
    for card in &cards {
        result += card.part_one_result;
    }
    result
}

fn part2() -> usize {
    let mut cards = parse_cards();
    let mut card_amount = cards.len();
    let mut index = 0;
    for i in 0..cards.len() {
        for copy in 0..(cards[i].copies + 1) {
            for win in 1..cards[i].winns + 1 {
                cards[i + win].copies += 1;
                card_amount += 1;
            }
        }

        index += 1;
    }
    card_amount
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}
