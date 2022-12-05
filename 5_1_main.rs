use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum ReadMode {
    Map,
    Instructions
}

#[derive(PartialEq, Debug, Clone)]
struct Map {
    stacks: Vec<Vec<char>>,
    num_stacks: usize
}

fn main() {
    let file_path = "./5_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    let mut read_mode = ReadMode::Map;
    let mut map_initialised = false;

    let mut map: Map = Map {
        num_stacks: 0,
        stacks: Vec::new()
    };

    for s in split {
        if s == "" {
            read_mode = ReadMode::Instructions;
            continue
        }
        if read_mode == ReadMode::Map {
            if !map_initialised {
                let num_stacks: usize = (s.len() + 1) / 4;

                for i in 0..num_stacks {
                    map.stacks.push(Vec::new());
                }
                map_initialised = true;
            }


            // println!("Map str len: {}", s.len());
            // println!("num of stacks {}", num_stacks);
        } else {
            // println!("Reading instruction");
        }
    }
    println!("Map initialised: {:?}", map);

    // println!("Score: {}", acc);
}