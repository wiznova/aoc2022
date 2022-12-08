use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

// pub trait MutChildren {
//     fn files_mut(&mut self) -> &mut Vec<File>;
// }

#[derive(Debug, Clone)]
struct File<'a> {
    name: &'a str,
    size: i32,
    parent: Option<Rc<RefCell<Folder<'a>>>>,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    name: &'a str,
    parent: Option<Rc<RefCell<Folder<'a>>>>,
    folders: Vec<Rc<RefCell<Folder<'a>>>>,
    files: Vec<Rc<RefCell<File<'a>>>>,
}

// impl MutChildren for Folder<'_> {
//     fn files_mut(&mut self) -> &mut Vec<File> {
//         &mut self.files
//     }
// }

fn is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn new_file<'a>(name: &'a str, size: i32) -> Rc<RefCell<File<'a>>> {
    Rc::new(RefCell::new(File {
        name: name,
        size: size,
        parent: None,
    }))
}
fn new_folder<'a>(name: &'a str) -> Rc<RefCell<Folder<'a>>> {
    Rc::new(RefCell::new(Folder {
        name: name,
        folders: Vec::new(),
        files: Vec::new(),
        parent: None,
    }))
}

fn main() {
    let file_path = "./7_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let split: Vec<&str> = contents.split("\n").collect();
    // let folders: Vec<Folder> = Vec::new();

    assert_eq!(split[0], "$ cd /");
    let root = new_folder("/");
    let mut current_folder: Rc<RefCell<Folder>> = root.clone();
    // let mut current_folder: &mut Folder = root;

    for s in &split[1..] {
        let s_split = s.split(" ");
        let mut s_vec = Vec::new();
        for part in s_split {
            s_vec.push(part);
        }
        // println!("{:?}", s_vec);
        match s_vec[..] {
            ["$", "ls"] => {
                println!("LS start");
                println!("{:?}", current_folder);
            },
            ["$", "cd", ".."] => {
                println!("cd back");
                let parent_folder = current_folder.borrow_mut().parent.clone();
                current_folder = parent_folder.unwrap();
            }
            ["$", "cd", path] => {
                println!("cd into: {}", path);
            }
            ["dir", dir_name] => {
                println!("dir: {}", dir_name);
                let new_folder = new_folder(dir_name);
                new_folder.borrow_mut().parent = Some(current_folder.clone());
                current_folder
                    .borrow_mut()
                    .folders
                    .push(new_folder);
            }
            [size, filename] if is_numeric(size) => {
                println!("file: {}-{}", filename, size);

                current_folder
                    .borrow_mut()
                    .files
                    .push(new_file(filename, size.parse::<i32>().unwrap()));
            }
            [] | [_] => todo!(),
            [&_, _] | [&_, _, _, ..] => todo!(),
        }
        // println!("Line: {}", s);
    }
    println!("{:?}", current_folder);
}
