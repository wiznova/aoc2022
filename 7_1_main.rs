use std::fs;

pub trait MutChildren {
    fn files_mut(&mut self) -> &mut Vec<File>;
}

#[derive(Debug, Copy, Clone)]
struct File<'a> {
    name: &'a str,
    size: i32,
    parent: Option<&'a Folder<'a>>,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    name: &'a str,
    parent: Option<&'a Folder<'a>>,
    folders: Vec<Folder<'a>>,
    files: Vec<File<'a>>,
}

impl MutChildren for Folder<'_> {
    fn files_mut(&mut self) -> &mut Vec<File> {
        &mut self.files
    }
}

fn is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn new_file<'a>(name: &'a str, size: i32, parent: &'a Folder) -> File<'a> {
    File {
        name: name,
        size: size,
        parent: Some(parent),
    }
}
fn new_folder<'a>(name: &'a str, parent: &'a Folder) -> Folder<'a> {
    Folder {
        name: name,
        folders: Vec::new(),
        files: Vec::new(),
        parent: Some(parent),
    }
}

fn main() {
    let file_path = "./7_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let split: Vec<&str> = contents.split("\n").collect();
    // let folders: Vec<Folder> = Vec::new();

    assert_eq!(split[0], "$ cd /");
    let mut root = Folder {
        name: "/",
        folders: Vec::new(),
        files: Vec::new(),
        parent: None,
    };
    let mut current_folder = &mut root;

    for s in &split[1..] {
        let s_split = s.split(" ");
        let mut s_vec = Vec::new();
        for part in s_split {
            s_vec.push(part);
        }
        // println!("{:?}", s_vec);
        match s_vec[..] {
            ["$", "ls"] => println!("LS start"),
            ["$", "cd", ".."] => {
                println!("cd back");
                current_folder = &mut current_folder.parent.unwrap();
            }
            ["$", "cd", path] => {
                println!("cd into: {}", path);
            }
            ["dir", dir_name] => {
                println!("dir: {}", dir_name);
                current_folder.folders
                .push(new_folder(dir_name, current_folder));
            }
            [size, filename] if is_numeric(size) => {
                println!("file: {}-{}", filename, size);
                current_folder
                    .files_mut()
                    .push(new_file(filename, size.parse::<i32>().unwrap(), current_folder));
            }
            [] | [_] => todo!(),
            [&_, _] | [&_, _, _, ..] => todo!(),
        }
        // println!("Line: {}", s);
    }
    println!("{:?}", root);
}
