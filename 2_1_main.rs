use std::fs;

fn main() {
    
    let file_path = "./2_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    for s in split {
        println!("{}", s);
    }


}
