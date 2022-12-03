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

    for s in split {
        let left = &s[0..s.len() / 2];
        let right = &s[s.len() / 2..];
        println!("part_1 {:?}", left);
        println!("part_2 {:?}", right);

        for ch in left.chars() {
            if right.contains(ch) {
                println!("Found! {} {}", ch, get_priority(ch));
                acc += get_priority(ch);
                break;
            }
        }
        // acc += calkculate_score(me, state) as i32;
    }
    
    
    println!("Score: {}", acc);

}
