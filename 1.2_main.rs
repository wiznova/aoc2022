// use std::env;
use std::fs;

fn main() {
    let file_path = "./1_input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let split: Vec<&str> = contents.split("\n").collect();
    let mut i = 0;

    for s in &split {
        if *s == "" {
            i += 1;
        }
    }

    let mut v = vec![1; i];
    println!("v length {}", v.len());


    let mut acc = 0;
    let mut num: i32;

    for s in split {
        if s == "" {
            v.push(acc);
            acc = 0;
            // println!("EMPTY {}", s);
        } else {
            num = s.parse().unwrap();
            acc += num;
            // println!("{}", acc);
        }
    }
    v.sort();
    println!("biggest: {}", v[v.len()-1]);
    println!("top 3: {}", v[v.len()-1] + v[v.len()-2] + v[v.len() - 3]);
    
}
