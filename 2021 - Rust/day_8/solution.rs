use std::fs::File;
use std::io::{BufRead, BufReader};

const PRODUCTION: bool = false;

fn part1() -> i32 {
    let dataset = read_dataset(false);
    let mut count_simple_digits = 0;

    for data_line in dataset.iter() {
        let char_count_10 = data_line[10].len();
        let char_count_11 = data_line[11].len();
        let char_count_12 = data_line[12].len();
        let char_count_13 = data_line[13].len();
        
        if char_count_10 == 2 || char_count_10 == 3 || char_count_10 == 4 || char_count_10 == 7 {
            count_simple_digits += 1;
        } 
        if  char_count_11 == 2 || char_count_11 == 3 || char_count_11 == 4 || char_count_11 == 7 {
            count_simple_digits += 1;
        } 
        if  char_count_12 == 2 || char_count_12 == 3 || char_count_12 == 4 || char_count_12 == 7 {
            count_simple_digits += 1;
        } 
        if  char_count_13 == 2 || char_count_13 == 3 || char_count_13 == 4 || char_count_13 == 7 {
            count_simple_digits += 1;
        }
        
    }
    
    count_simple_digits
}



fn part2() -> i32 {
    let array_filler: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let dataset = read_dataset(false);
    let mut result_vec: Vec<[String; 14]> = vec!();
    let result: i32;

    for data_line in dataset.iter() {

        let mut index = 0;
        let mut possibilities: [[char; 7]; 7] = [array_filler; 7];
        let mut decoded = false;
        // let mut result : [String; 14];
        
        print!("line: {:?}\n", data_line);
        print!("possibilities: {:?}\n", possibilities);
        while !decoded {
            for (i, x) in data_line.iter().enumerate() {
                if x.len() == 2 {
                    for (j, segment) in possibilities.iter().enumerate() {
                        for (k, segment_connection) in segment.iter().enumerate() {

                            for segment_connection_input in x.chars() {
                                if segment_connection_input != possibilities[j][k] {
                                    possibilities[j][k] = 't';      
                                }
                            }
                        }
                    }
                    //remove everything from 3 and 6 except from x content
                    // possibilities.retain(|&y| y == x.0 || y == x.1 );
                }
            }
        }
    }
    
    result = -1;
    result
}

// fn remove_possibilities(&self, delete_vec: Vec<char>) -> [Vec<char>; 14]{
//     let mut return_value: [Vec<char>; 14] = [array_filler; 14];

//     for (i, input) in self.iter() {
//         for (j, output) in delete_vec {
//             if input == output {

//             }
//         }
//     }
// }

fn read_dataset(_verbose: bool) -> Vec<[String; 14]> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input_vec: Vec<[String; 14]> = vec!();
    let mut current_digit: String = String::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut index = 0;
        input_vec.push([String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()]);
        let input_vec_length = input_vec.len();
        for character in line.chars() {
            if character != '|' && character != ' ' {
                current_digit.push(character);
            } else if character == ' ' {
                if current_digit.len() > 0 {
                    input_vec[input_vec_length - 1][index] = current_digit.to_string();
                    index += 1;
                }
                current_digit = String::new();
            }
        }        
        input_vec[input_vec_length - 1][13] = current_digit.to_string();
        current_digit = String::new();
    }

    if _verbose {
        for x in input_vec.iter() {
            print!("{:?}\n", x);
            print!("{:?}\n", x[13].len());
        }
    }
    input_vec
}

fn main() {
    let solution_part_1 = part1();
    if PRODUCTION {
        print!("Calculating for PRODUCTION\n");
        assert_eq!(solution_part_1, 456);
    } else {
        print!("Calculating for SAMPLE\n");
        assert_eq!(solution_part_1, 26);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let solution_part_2 = part2();
    if PRODUCTION {
        assert_eq!(solution_part_2, 61229);
    } else {
        assert_eq!(solution_part_2, -1);
    }
    print!("Part 2: {:?}\n", solution_part_2);
}