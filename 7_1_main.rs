use std::fs;

#[derive(Debug, Copy, Clone)]
struct File<'a> {
    name: &'a str,
    size: i32,
    parent: &'a Folder<'a>,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    name: &'a str,
    parent: Option<&'a Folder <'a>>,
    folders: Vec<&'a Folder<'a>>,
    files: Vec<&'a File<'a>>,
}

fn is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn main() {
    let file_path = "./7_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let split: Vec<&str> = contents.split("\n").collect();

    assert_eq!(split[0], "$ cd /");
    let root = Folder{
        name: "/",
        folders: Vec::new(),
        files: Vec::new(),
        parent: None
    };

    for s in &split[1..] {
        let s_split = s.split(" ");
        let mut s_vec = Vec::new();
        for part in s_split {
            s_vec.push(part);
        }
        // println!("{:?}", s_vec);
        match s_vec[..] {
            ["$", command, arg1, ..] => println!("command: {} {}", command, arg1),
            ["$", command, ..] => println!("command: {}", command),
            ["dir", dir_name, ..] => println!("directory: {}", dir_name),
            [size, filename, ..] if is_numeric(size) => println!("file: {}-{}", filename, size),
            [] | [_] | [_, _, _, _, ..] => todo!(),
            [&_, _] | [&_, _, _] => todo!()
        }
        // println!("Line: {}", s);
    }
    println!("{:?}", root);
}
