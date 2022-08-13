use std::fs::File;
use std::io::{BufRead, BufReader};

const PRODUCTION: bool = true;
const DIMENSIONS: (i8, i8) = (10, 10);

fn part1() -> u64 {
    let mut dataset = read_dataset(false);
    let mut return_value = 0;
    
    for _ in 0..100 {
        //bumping every index from -1 to 0
        for y in 0..DIMENSIONS.0 + 1 {
            for x in 0..DIMENSIONS.1 + 1 {
                if y >= 0 && y < DIMENSIONS.1 && x >= 0 && x < DIMENSIONS.0 {
                    if dataset[y as usize][x as usize] == -1 {
                        dataset[y as usize][x as usize] = 0;
                    }
                }
            }
        }

        //startup dfs
        for y in 0..DIMENSIONS.0 + 1 {
            for x in 0..DIMENSIONS.1 + 1 {
                if y >= 0 && y < DIMENSIONS.1 && x >= 0 && x < DIMENSIONS.0 {
                    //directions 0: up, 1: right, 2: down, 3: left, 4: up-left, 5: up-right, 6: down-right, 7: down-left, 8: all
                    let return_val = dfs((y, x), &dataset);
                    return_value += return_val.0;
                    dataset = return_val.1;
                }
            }
        }
    }
    return_value
}
            
            
fn dfs(point: (i8, i8), dataset: &Vec<Vec<i8>>) -> (u64, Vec<Vec<i8>>) {
    let mut return_value: (u64, Vec<Vec<i8>>) = (0, dataset.to_vec());
    if point.0 < 0 || point.0 > DIMENSIONS.0 - 1 || point.1 < 0 || point.1 > DIMENSIONS.1 - 1 {
        return return_value;
    }
    if return_value.1[point.0 as usize][point.1 as usize] == -1 {
        return return_value;
    } else {
        return_value.1[point.0 as usize][point.1 as usize] += 1;
        if return_value.1[point.0 as usize][point.1 as usize] > 9 {

            return_value.1[point.0 as usize][point.1 as usize] = -1;
            
            return_value.0 += 1;
            
            let mut returned: (u64, Vec<Vec<i8>>);
            
            returned = dfs((point.0 + 1, point.1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0, point.1 + 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0 - 1, point.1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0, point.1 - 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0 - 1, point.1 - 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0 + 1, point.1 - 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0 + 1, point.1 + 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;
            
            returned = dfs((point.0 - 1, point.1 + 1), &return_value.1.to_vec());
            return_value.0 += returned.0;
            return_value.1  = returned.1;       
        }
    }
    return_value
}
    
    
fn part2() -> u64 {
    let mut dataset = read_dataset(false);
    let mut return_value = 0;
    let mut return_checker = 0;        
    
    while return_checker != 100 {
        return_checker = 0;
        
        //bumping every index from -1 to 0
        for y in 0..DIMENSIONS.0 + 1 {
            for x in 0..DIMENSIONS.1 + 1 {
                if y >= 0 && y < DIMENSIONS.1 && x >= 0 && x < DIMENSIONS.0 {
                    if dataset[y as usize][x as usize] == -1 {
                        dataset[y as usize][x as usize] = 0;
                    }
                }
            }
        }
        //startup dfs
        for y in 0..DIMENSIONS.0 + 1 {
            for x in 0..DIMENSIONS.1 + 1 {
                if y >= 0 && y < DIMENSIONS.1 && x >= 0 && x < DIMENSIONS.0 {
                    //directions 0: up, 1: right, 2: down, 3: left, 4: up-left, 5: up-right, 6: down-right, 7: down-left, 8: all
                    let return_val = dfs((y, x), &dataset);
                    return_checker += return_val.0;
                    dataset = return_val.1;
                }
            }
        }
        return_value += 1;
    }
    return_value
}
    
fn read_dataset(_verbose: bool) -> Vec<Vec<i8>> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<i8>> = vec!();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input.push(vec!());
        for character in line.chars() {
            input[i].push(character as i8 - 0x30);
        }        
        if _verbose { print!("{:?}\n", input[i]); }
    }
    input
}

fn main() {
    if PRODUCTION {
        print!("PRDOUCTION\n");
    } else {
        print!("DEVELOPMENT\n");
    }
    let solution_part_1 = part1();
    if PRODUCTION {
        assert_eq!(solution_part_1, 1721);
    } else {
        assert_eq!(solution_part_1, 1656);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let solution_part_2 = part2();
    if PRODUCTION {
        assert_eq!(solution_part_2, 298);
    } else {
        assert_eq!(solution_part_2, 195);
    }
    print!("Part 2: {}\n", solution_part_2);
}