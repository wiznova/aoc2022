use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::thread::current;

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
        size_files: 0,
        size_cumul: 0,
    }))
}

fn folder_size(folder: &Rc<RefCell<Folder>>) -> u64 {
    let mut size = folder.borrow().size_files;
    for f in &folder.borrow().folders {
        size += folder_size(&f);
    }
    return size;
}

// - / (dir)
//   - d (dir)
//     - j (file, size=4060174)
//     - d.log (file, size=8033020)
//     - d.ext (file, size=5626152)
//     - k (file, size=7214296)
//   - a (dir)
//     - e (dir)
//       - i (file, size=584)
//     - f (file, size=29116)
//     - g (file, size=2557)
//     - h.lst (file, size=62596)
//   - b.txt (file, size=14848514)
//   - c.dat (file, size=8504156)

fn print_tree(root: &Rc<RefCell<Folder>>, mut depth: usize) {
    println!("{:<depth$}- {} (dir)", " ", root.borrow().name);
    depth = depth + 1;
    for file in &root.borrow().files {
        println!(
            "{:<depth$}- {} (file, size={})",
            " ",
            file.borrow().name,
            file.borrow().size
        );
    }
    for folder in &root.borrow().folders {
        print_tree(folder, depth);
    }
}

fn calculate_ch1(root: &Rc<RefCell<Folder>>, upper_bound: u64) -> u64 {
    let mut sum = 0;
    let folder_size = folder_size(root);
    println!(
        "Checking folder: {}, size: {folder_size}",
        root.borrow().name
    );
    if folder_size <= upper_bound {
        sum += folder_size;
    }
    for f in &root.borrow().folders {
        println!(
            "\tChecking subfolder: {}, size: {folder_size}",
            f.borrow().name
        );
        sum += calculate_ch1(&f, upper_bound);
    }
    sum
}

fn main() {
    let file_path = "./7_input.txt";
    // let file_path = "./7_input_test.txt";
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
                current_folder = current_folder_copy
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .clone();
            }
            ["$", "cd", path] => {
                println!("cd into: {}", path);
                let new_folder = new_folder(path);
                new_folder.borrow_mut().parent = Some(current_folder.clone());
                current_folder.borrow_mut().folders.push(new_folder.clone());
                current_folder = new_folder.clone();
            }
            [size, filename] if is_numeric(size) => {
                println!("file: {}-{}", filename, size);
                current_folder.borrow_mut().size_files += size.parse::<u64>().unwrap();

                current_folder
                    .borrow_mut()
                    .files
                    .push(new_file(filename, size.parse::<i32>().unwrap()));
            }
            _ => (),
        }
        // println!("Line: {}", s);
    }

    //Print tree
    print_tree(&root, 1);

    let ch1_result = calculate_ch1(&root, 100000u64);
    println!("ch1_result: {ch1_result}");
    // println!("{:?}", root.borrow().size_files);
    // println!("{:?}", root.borrow().folders[0]);
}
