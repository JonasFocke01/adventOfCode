use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() -> u32 {
    let data = read_dataset(true, false);
    let mut result: u32 = 0;

    for y in data.iter() {
        for x in y.iter() {
            if *x > 1 {
                result += 1;
            }
        }
    }
    
    result
}

fn part2() -> u32 {
    let data = read_dataset(false, false);
    let mut result: u32 = 0;

    for y in data.iter() {
        for x in y.iter() {
            if *x > 1 {
                result += 1;
            }
        }
    }
    
    result
}

fn read_dataset(read_only_straight_lines: bool, verbose: bool) -> Vec<Vec<i16>> {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut current_char = String::new();

    let mut vec_vec: Vec<[[i16; 2]; 2]> = vec!(); // the parsed input
    let mut removables: Vec<u16> = vec!();
    let mut return_vec: Vec<Vec<i16>> = vec!();

    for (i, line) in reader.lines().enumerate() {
        vec_vec.push([[-1, -1], [-1, -1]]);
        let line = line.unwrap();
        for character in line.chars() {
            if character.is_numeric() == true {
                current_char.push(character);
            } else if current_char.len() > 0 {
                if vec_vec[i][0][0] == -1 {
                    vec_vec[i][0][0] = current_char.parse::<i16>().unwrap();
                } else if vec_vec[i][0][1] == -1 {
                    vec_vec[i][0][1] = current_char.parse::<i16>().unwrap();
                } else if vec_vec[i][1][0] == -1 {
                    vec_vec[i][1][0] = current_char.parse::<i16>().unwrap();
                }

                current_char = String::from("");
            }
        }
        vec_vec[i][1][1] = current_char.parse::<i16>().unwrap();
        current_char = String::from("");
        
    }
    for _x in 0..1000 {
        return_vec.push(vec!());
    }
    for (i, _x) in (0..return_vec.len()).enumerate() {
        for _y in 0..return_vec.len() {
            return_vec[i].push(0);
        }
    }

    if read_only_straight_lines == true {
        for (i, point_pair) in vec_vec.iter().enumerate() {
            if point_pair[0][0] == point_pair[1][0] || point_pair[0][1] == point_pair[1][1] {
                if verbose == true { print!("scipping {:?}\n", point_pair); }
            } else {
                removables.push(i as u16);
            }
        }
        for remove_index in removables.iter().rev() {
            if verbose == true { print!("removing: {:?}\n", vec_vec[*remove_index as usize]) }
            vec_vec.remove(*remove_index as usize);
        }
    }

    for x in vec_vec.iter() {
        let appart: i16;
        if (x[0][0] - x[1][0]).abs() >= (x[0][1] - x[1][1]).abs() {
            appart = (x[0][0] - x[1][0]).abs();
        } else {
            appart = (x[0][1] - x[1][1]).abs();
        }
        if verbose == true { print!("{:?} - {}\n", x, appart); }
    }

    if verbose { print!("return_vec len({})\n", return_vec.len()); }
    for e in vec_vec.iter_mut() {

        let appart: i16;
        if (e[0][0] - e[1][0]).abs() >= (e[0][1] - e[1][1]).abs() {
            appart = (e[0][0] - e[1][0]).abs();
        } else {
            appart = (e[0][1] - e[1][1]).abs();
        }
        for _el in 0..(appart + 1) {
            return_vec[e[0][0] as usize][e[0][1] as usize] += 1;
            if e[0][0] < e[1][0] {
                e[0][0] += 1;
            } else if e[0][0] > e[1][0] {
                e[0][0] -= 1;
            }

            if e[0][1] < e[1][1] {
                e[0][1] += 1;
            } else if e[0][1] > e[1][1] {
                e[0][1] -= 1;
            }
        }
    }

    if verbose {
        for x in return_vec.iter() {
            for (j, y) in x.iter().enumerate() {
                if *y > 0 {
                    print!("[{:1}]", x[j]);
                } else {
                    print!("{:2} ", x[j]);
                }
            }
            print!("\n");
        } 
    }
    
    return_vec
}

fn main() {
    print!("Part 1: {}\n", part1());
    print!("Part 2: {}\n", part2());
}