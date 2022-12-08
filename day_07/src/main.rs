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
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    name: &'a str,
    parent: Option<Rc<RefCell<Folder<'a>>>>,
    folders: Vec<Rc<RefCell<Folder<'a>>>>,
    files: Vec<Rc<RefCell<File<'a>>>>,
    size_files: u64,
    size_cumul: u64,

}

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
            ["$", "cd", ".."] => {
                println!("cd back");
                // let parent_folder = current_folder.borrow_mut().parent.clone();
                let current_folder_copy = current_folder.clone();
                current_folder = current_folder_copy.borrow().parent.as_ref().unwrap().clone();
            }
            ["$", "cd", path] => {
                println!("cd into: {}", path);
                let new_folder = new_folder(path);
                new_folder.borrow_mut().parent = Some(current_folder.clone());
                current_folder.borrow_mut().folders.push(new_folder.clone());

                // let into_folder = current_folder.borrow_mut().with_name("path");
                current_folder = new_folder.clone();
            }
            [size, filename] if is_numeric(size) => {
                println!("file: {}-{}", filename, size);

                current_folder
                    .borrow_mut()
                    .files
                    .push(new_file(filename, size.parse::<i32>().unwrap()));
            },
            _ => (),
        }
        // println!("Line: {}", s);
    }
    // println!("{:?}", current_folder);
}
