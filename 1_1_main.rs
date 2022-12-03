// use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = "./1_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut split = contents.split("\n");
    let mut acc = 0;
    let mut num = 0;
    let mut biggest = 0;

    for s in split {        
        if s == "" {
            if acc > biggest {
                biggest = acc;
            }
            acc = 0;
            println!("EMPTY {}", s);
        } else {
            num = s.parse().unwrap();
            acc += num;
            println!("{}", acc);
        }
    }
    println!("biggest: {}", biggest);

    // println!("With text:\n{split}");
}
