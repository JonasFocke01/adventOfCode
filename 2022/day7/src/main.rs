use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

#[derive(Clone)]
struct Directory {
    sub_dirs: Vec<Directory>,
    files: Vec<TreeFile>,
    name: String,
    size: i32,
    doubled_sub_dir_size: i32,
}

impl Directory {
    fn display(&self, indent_level: u8) {
        let mut indent_string = String::new();
        (0..indent_level).for_each(|_| indent_string.push(' '));
        println!("{}{}/: {}", indent_string, self.name, self.size);
        self.sub_dirs
            .iter()
            .for_each(|sub_dir| sub_dir.display(indent_level + 2));
        self.files
            .iter()
            .for_each(|file| print!("{}  {} {}\n", indent_string, file.name, file.size));
    }
    fn insert_dir(&mut self, insert_dir: Directory) {
        self.sub_dirs.push(insert_dir);
    }

    fn insert_file(&mut self, insert_file: TreeFile) {
        self.files.push(insert_file);
    }
    fn find_sub_dir(&mut self, name: &str) -> Option<&mut Directory> {
        let self_name = self.name.to_string();
        if self_name == name {
            return Some(self);
        } else {
            let mut found_dir = None;
            self.sub_dirs.iter_mut().for_each(|dir| {
                if let Some(e) = dir.find_sub_dir(name) {
                    found_dir = Some(e);
                }
            });
            return found_dir;
        }
    }
    fn determine_all_dir_sizes(&mut self) {
        self.sub_dirs
            .iter_mut()
            .for_each(|dir| dir.determine_all_dir_sizes());

        self.size = self.get_size(false);
        self.doubled_sub_dir_size = self.get_size(true);
    }
    fn get_size(&self, double_sub_dirs: bool) -> i32 {
        let sub_dir_size = if double_sub_dirs {
            self.sub_dirs.iter().map(|dir| dir.size).sum::<i32>() * 2
        } else {
            self.sub_dirs.iter().map(|dir| dir.size).sum::<i32>()
        };
        sub_dir_size + self.files.iter().map(|file| file.size).sum::<i32>()
    }
    fn get_total_size_by_min_size(&self, min_size: i32) -> i32 {
        let mut sum = self.size;
        self.sub_dirs
            .iter()
            .for_each(|sub_dir| sum += sub_dir.doubled_sub_dir_size);
        if sum >= min_size {
            sum
        } else {
            0
        }
    }
}

#[derive(Clone)]
struct TreeFile {
    size: i32,
    name: String,
}

fn part1() -> i32 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_path: Vec<String> = vec![String::from("/")];

    let mut tree: Vec<Directory> = vec![Directory {
        sub_dirs: vec![],
        files: vec![],
        name: "/".to_string(),
        size: 0,
        doubled_sub_dir_size: 0,
    }];

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return;
        };

        let mut line = line.split(' ');
        let first_char = line.next().unwrap();
        if first_char == "$" {
            if line.next().unwrap() == "cd" {
                let dir_char = line.next().unwrap().to_string();
                if dir_char == "/" {
                    current_path = vec![String::from("/")];
                } else if dir_char == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir_char);
                }
            }
        } else if first_char == "dir" {
            tree[0]
                .find_sub_dir(current_path.last().unwrap())
                .unwrap()
                .insert_dir(Directory {
                    name: line.next().unwrap().to_string(),
                    files: vec![],
                    sub_dirs: vec![],
                    size: 0,
                    doubled_sub_dir_size: 0,
                });
        } else {
            tree[0]
                .find_sub_dir(current_path.last().unwrap())
                .unwrap()
                .insert_file(TreeFile {
                    size: first_char.parse::<i32>().unwrap(),
                    name: line.next().unwrap().to_string(),
                });
        }
    });

    tree[0].determine_all_dir_sizes();
    tree[0].display(0);

    // determine all dirs of size greater than 100000
    tree[0].get_total_size_by_min_size(100000)
}

fn part2() -> String {
    String::new()
}

//fn print_file_tree(root: &Directory, indent: u8) {
//print!("----");
//let mut indent_string = String::new();
//(0..indent).for_each(|i| indent_string.push(' '));
//print!("{}{}\n", indent_string, root.name);
//root.files
//.iter()
//.for_each(|file| print!("{}{} {}\n", indent_string, file.name, file.size));
//root.sub_dirs
//.iter()
//.for_each(|sub_dir| print_file_tree(sub_dir, indent + 2));
//print!("----");
//}

fn main() {
    println!("result_1: {result_1}", result_1 = part1());
    println!("result_2: {result_2}", result_2 = part2());
}
