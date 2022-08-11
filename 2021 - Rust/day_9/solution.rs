use std::fs::File;
use std::io::{BufRead, BufReader};

const PRODUCTION: bool = true;
const RANGE_X: u16 = 99;
const RANGE_Y: u16 = 100;
// const RANGE_X: u16 = 9;
// const RANGE_Y: u16 = 5;

fn part1() -> u16 {
    let dataset = read_dataset(false);
    let mut return_value: u16 = 0 ;

    for (i, row) in dataset.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            return_value += check_if_low_point((i as u16, j as u16), &dataset);
        }
    }

    return_value
}



fn part2() -> u32 {
    let dataset = read_dataset(false);
    
    let mut dataset_info_if_checked: Vec<Vec<(u8, bool)>> = vec!();
    let mut temp_dataset_info_if_checked: Vec<Vec<(u8, bool)>>;
    let mut basins: Vec<u16> = vec!();


    for (i, row) in dataset.iter().enumerate() {
        dataset_info_if_checked.push(vec!());
        for (j, _point) in row.iter().enumerate() {
            dataset_info_if_checked[i].push((dataset[i as usize][j as usize], false));
        }
    }

    //logic to build the basins
    temp_dataset_info_if_checked = dataset_info_if_checked.to_vec();
    for index in 0..9 {
        for (i, row) in dataset_info_if_checked.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                if column.0 == index && column.1 == false {
                    let ret_val = check_if_basin((i as u16, j as u16), temp_dataset_info_if_checked);
                    basins.push(ret_val.0);
                    temp_dataset_info_if_checked = ret_val.1;
                }
            }
        }
        dataset_info_if_checked = temp_dataset_info_if_checked.to_vec();
    }

    //find greatest three in dataset
    let mut greatest_1: u16 = u16::MIN;
    let mut greatest_2: u16 = u16::MIN;
    let mut greatest_3: u16 = u16::MIN;

    for basin in basins.iter() {
        if basin > &greatest_1 {
            greatest_2 = greatest_1;
            greatest_3 = greatest_2;
            greatest_1 = *basin;
        }
        if basin > &greatest_2 && basin != &greatest_1 {
            greatest_3 = greatest_2;
            greatest_2 = *basin;
        }
        if basin > &greatest_3 && basin != &greatest_1 && basin != &greatest_2 {
            greatest_3 = *basin;
        }
    }

    greatest_1 as u32 * greatest_2 as u32 * greatest_3 as u32
}

fn check_if_basin(point: (u16, u16), mut input: Vec<Vec<(u8, bool)>>) -> (u16, Vec<Vec<(u8, bool)>>) {
    let mut return_value: u16 = 1;

    input[point.0 as usize][point.1 as usize].1 = true;

    //left
    if point.1 > 0 && !input[point.0 as usize][point.1 as usize - 1].1 && input[point.0 as usize][point.1 as usize - 1].0 != 9 {
        let ret_val = check_if_basin((point.0 as u16, point.1 - 1 as u16), input.to_vec());
        return_value += ret_val.0;
        input = ret_val.1;
    }

    //right
    if point.1 < RANGE_X && !input[point.0 as usize][point.1 as usize + 1].1 && input[point.0 as usize][point.1 as usize + 1].0 != 9 {
        let ret_val = check_if_basin((point.0, point.1 + 1), input.to_vec());
        return_value += ret_val.0;
        input = ret_val.1;
    }

    //up
    if point.0 > 0 && !input[point.0 as usize - 1][point.1 as usize ].1 && input[point.0 as usize - 1][point.1 as usize].0 != 9 {
        let ret_val = check_if_basin((point.0 as u16 - 1, point.1 as u16), input.to_vec());
        return_value += ret_val.0;
        input = ret_val.1;
    }

    //down
    if point.0 < RANGE_Y - 1 && !input[point.0 as usize + 1][point.1 as usize].1 && input[point.0 as usize + 1][point.1 as usize].0 != 9 {
        let ret_val = check_if_basin((point.0 + 1 as u16, point.1 as u16), input.to_vec());
        return_value += ret_val.0;
        input = ret_val.1;
    }

    (return_value, input)
}

fn check_if_low_point(point: (u16, u16), input: &Vec<Vec<u8>>) -> u16 {
    let value = input[point.0 as usize][point.1 as usize];
    let mut found = 0;
    if point.0 > 0 {
        if input[point.0 as usize - 1][point.1 as usize] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.1 > 0 {
        if input[point.0 as usize][point.1 as usize - 1] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.0 < RANGE_Y - 1 {
        if input[point.0 as usize + 1][point.1 as usize] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if point.1 < RANGE_X {
        if input[point.0 as usize][point.1 as usize + 1] > value {
            found += 1;
        }
    } else {
        found += 1;
    }

    if found == 4 {
        return input[point.0 as usize][point.1 as usize] as u16 + 1;
    } else {
        return 0;
    }
}

fn read_dataset(_verbose: bool) -> Vec<Vec<u8>> {
    let filename: String;
    if PRODUCTION {
        filename = String::from("input.txt");
    } else {
        filename = String::from("sample.txt");
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<u8>> = vec!();
    
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        input.push(vec!());
        for character in line.chars() {
            input[i].push(character as u8 - 0x30);
        }        
        if _verbose { print!("linebreak\n"); }
    }
    input
}

fn main() {
    let solution_part_1 = part1();
    if PRODUCTION {
        assert_eq!(solution_part_1, 528);
    } else {
        assert_eq!(solution_part_1, 15);
    }
    print!("Part 1: {:?}\n", solution_part_1);
    
    let solution_part_2 = part2();
    if PRODUCTION {
        assert_eq!(solution_part_2, 920448);
    } else {
        assert_eq!(solution_part_2, 1134);
    }
    print!("Part 2: {}\n", solution_part_2);
}