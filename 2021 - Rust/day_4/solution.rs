use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> i32 {
    let data = read_dataset();
    let mut field_data = read_dataset(); 
    let mut temp_counter: u8 = 0;
    let mut winner: Option<&(i32, Vec<(u8, u8, u8, bool)>)> = None;
    let mut solution: i32 = 0;
    
    for solution_piece in data.0.iter() {
        for (i, riddle) in data.1.iter().enumerate() {
            for (j, riddle_piece) in data.1[i].1.iter().enumerate() {
                if riddle_piece.0 == *solution_piece {
                    field_data.1[i].1[j].3 = true;
                }
            }
        }
        winner = check_if_winner_is_found(&field_data.1);
        if winner == None {
            print!("\ncontinue search for winner...\n");
        } else {
            for e in winner.unwrap().1.iter() {
                print!("{:?}\n", e);
                if e.3 == false {
                    solution += e.0 as i32;
                }
            }
            return solution * *solution_piece as i32;
        }
    }
    -1
}

fn part2() -> i32 {
    -1
}

fn check_if_winner_is_found( suspectVec: &Vec<(i32, Vec<(u8, u8, u8, bool)>)> ) -> Option<&(i32, Vec<(u8, u8, u8, bool)>)> {    
    
    let mut hits_x: u8 = 0;
    let mut hits_y: [u8; 5] = [0, 0, 0, 0, 0];
    let mut current_line = 0;

    for (i, e) in suspectVec.iter().enumerate() {
        print!("\n\n------- {} -------\n", e.0);
        for (j, el) in e.1.iter().enumerate() {
            if current_line == 5 && el.2 == 1 {
                current_line = 1;
                print!("\n");
                hits_x = 0;
            }
            if current_line < el.2 {
                current_line += 1;
                print!("\n");
                hits_x = 0;
            }
            if el.3 == true {
                hits_y[j % 5] += 1;
                hits_x += 1;
                print!("\x1b[93m{:2} \x1b[0m", el.0);
            } else {
                print!("{:2} ", el.0);
            }
            if hits_x == 5 || hits_y.contains(&5) == true {
                print!("\n\n\n\n");
                return Some(e);
            }
        }
        hits_y = [0, 0, 0, 0, 0];
    }

    None
}

/**
 * @return:
 * touple ( Vec< RIDDLE_SOLUTIONS > , Vec<  touple( RIDDLE_NUMBER , VALUE , X , Y , MARKED ) > )
 */
fn read_dataset() -> (Vec<u8>, Vec<(i32, Vec<(u8, u8, u8, bool)>)>){

    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut output: (Vec<u8>, Vec<(i32, Vec<(u8, u8, u8, bool)>)>) = (vec!(), vec!());
    let mut y: u8 = 1;
    let mut x: u8 = 1;
    let mut riddle: i32 = 0;
    
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        if output.1.len() == 0 || y == 5 {
            output.1.push((riddle, vec!()));
        }
        let mut current_number: String = String::new();
        
        if _i == 0 {
            for char in line.chars() {
                if char.is_numeric() == true {
                    current_number.push(char);
                } else {
                    output.0.push(current_number.parse::<u8>().unwrap());
                    current_number = "".to_string();
                }
            }
            output.0.push(current_number.parse::<u8>().unwrap());
        } 
        
        else if _i > 1 && line.len() > 0 {
            for char in line.chars() {
                if char.is_numeric() == true {
                    current_number.push(char);
                } else if current_number.len() > 0 {
                    output.1[riddle as usize].1.push((current_number.parse::<u8>().unwrap(), x, y, false));
                    if x == 5 {
                        x = 1;
                    } else {
                        x += 1;
                    }
                    current_number = "".to_string();
                }
            }
            output.1[riddle as usize].1.push((current_number.parse::<u8>().unwrap(), x, y, false));
            if y == 5 {
                riddle += 1;
            }
            if x == 5 {
                x = 1;
            } else {
                x += 1;
            }
            if y == 5 {
                y = 1;
            } else {
                y += 1;
            }
        }
    }

    output.1.pop();
    output
}

fn main() {
    print!("Part 1: {:?}\n", part1());
    print!("Part 2: {}\n", part2());
}