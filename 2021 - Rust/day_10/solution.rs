use std::fs::File;
use std::io::{BufRead, BufReader};

const PRODUCTION: bool = true;
const OPENING_BRACKETS: [char; 4] = ['(', '{', '[', '<'];
const CLOSING_BRACKETS: [char; 4] = [')', '}', ']', '>'];

fn part1() -> u64 {
    let dataset = read_dataset(false);
    let mut return_value: u64 = 0 ;
    
    for line in dataset.iter() {
        let mut line_bracket_stack: Vec<char> = vec!();
        'this_line: for character in line.iter() {
            if OPENING_BRACKETS.iter().position(|&x| x == *character) != None {
                line_bracket_stack.push(*character);
            } else {
                let value_from_stack = line_bracket_stack.pop().unwrap();
                if OPENING_BRACKETS.iter().position(|&x| x == value_from_stack).unwrap() != CLOSING_BRACKETS.iter().position(|&x| x == *character).unwrap(){
                    if *character == ')' {
                        return_value += 3;
                        break 'this_line;
                    }
                    if *character == ']' {
                        return_value += 57;
                        break 'this_line;
                    }
                    if *character == '}' {
                        return_value += 1197;
                        break 'this_line;
                    }
                    if *character == '>' {
                        return_value += 25137;
                        break 'this_line;
                    }

                }
            }
        }
    }

    return_value
}



fn part2() -> u64 {
    let dataset = read_dataset(false);
    let mut score_vec: Vec<u64> = vec!();
    let mut return_value: u64 = 0 ;
    
    for line in dataset.iter() {
        let mut line_bracket_stack: Vec<char> = vec!();
        let mut breaking = false;
        'this_line: for character in line.iter() {
            if OPENING_BRACKETS.iter().position(|&x| x == *character) != None {
                line_bracket_stack.push(*character);
            } else {
                let value_from_stack = line_bracket_stack.pop().unwrap();
                // print!("{:?} | {:?}\n", OPENING_BRACKETS.iter().position(|&x| x == value_from_stack).unwrap(), 
                // CLOSING_BRACKETS.iter().position(|&x| x == *character).unwrap());
                if OPENING_BRACKETS.iter().position(|&x| x == value_from_stack).unwrap() != CLOSING_BRACKETS.iter().position(|&x| x == *character).unwrap(){
                    breaking = true;
                    break 'this_line;
                }
            }
        }
        if !breaking && line_bracket_stack.len() > 0 {
            let mut closing_brackets: Vec<char> = vec!();
            let line_bracket_stack_length = line_bracket_stack.len();
            for x in 0..line_bracket_stack_length {
                let value_from_stack = line_bracket_stack.pop().unwrap();
                closing_brackets.push(CLOSING_BRACKETS[OPENING_BRACKETS.iter().position(|&x| x == value_from_stack).unwrap()]);
            }
            breaking = false;
            let mut score: u64 = 0;
            for bracket in closing_brackets {
                score *= 5;
                if bracket == ')' {
                    score += 1;
                } else if bracket == ']' {
                    score += 2;
                } else if bracket == '}' {
                    score += 3;
                } else if bracket == '>' {
                    score += 4;
                }
            }
            score_vec.push(score);
        }
    }
    score_vec.sort();
    
    score_vec[score_vec.len() / 2]
}

fn read_dataset(_verbose: bool) -> Vec<Vec<char>> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<char>> = vec!();
    
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input.push(vec!());
        for character in line.chars() {
            input[i].push(character);
        }        
        if _verbose { print!("{:?}\n", input[i]); }
    }
    input
}

fn main() {
    let solution_part_1 = part1();
    if PRODUCTION {
        assert_eq!(solution_part_1, 358737);
    } else {
        assert_eq!(solution_part_1, 26397);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let solution_part_2 = part2();
    if PRODUCTION {
        assert_eq!(solution_part_2, 4329504793);
    } else {
        assert_eq!(solution_part_2, 288957);
    }
    print!("Part 2: {}\n", solution_part_2);
}