use std::collections::HashSet;
use std::fs;

fn count_unique(list: &[char]) -> usize {
    let mut set = HashSet::new();
    for &x in list {
        set.insert(x);
    }
    set.len()
}

fn main() {
    let file_path = "./6_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let window_size = 14;
    // let split: Vec<&str> = contents.split("\n").collect();

    for (i, window) in contents
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        // println!("{} {:?}", i, window);
        if count_unique(window) == window_size {
            println!("{} {:?}", i, window);
            println!("idx = {}", (i + window_size));
            break
        }
    }
}
