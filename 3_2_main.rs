use std::fs;

fn get_priority(ch: char) -> u32 {
    if ch.is_lowercase() {
        ch as u32 - 96
    } else {
        ch as u32 - 38
    }
}

fn main() {
    let file_path = "./3_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut acc: u32 = 0;
    let mut i: usize = 0;

    while i + 2 < 300 {
        let first = &split[i];
        let second = &split[i+1];
        let third = &split[i+2];


        for ch in first.chars() {
            if second.contains(ch) && third.contains(ch) {
                println!("Found! {} {}", ch, get_priority(ch));
                acc += get_priority(ch);
                break;
            }
        }

        i += 3;
        println!("{}", i);
    }

    println!("Score: {}", acc);
}
