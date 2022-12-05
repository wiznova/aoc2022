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
            for stack in &mut map.stacks {
                stack.reverse();
            }
            println!("Map initialised: {:?}", map);
            continue
        }
        if read_mode == ReadMode::Map {
            if !map_initialised {
                map.num_stacks = (s.len() + 1) / 4;
                
                for _ in 0..map.num_stacks {
                    map.stacks.push(Vec::new());
                }
                map_initialised = true;
            }
            for (stack_i, chars) in s.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if chars[1] != ' ' && !chars[1].is_numeric() {
                    map.stacks[stack_i].push(chars[1])
                }
            }
        } else {
            let halves: Vec<&str> = s.split(" from ").collect();
            let quantity = halves[0][5..halves[0].len()].parse::<i32>().unwrap();
            let from_i = (halves[1][0..1].parse::<i32>().unwrap() - 1) as usize;
            let to_i = (halves[1][halves[1].len() - 1..halves[1].len()].parse::<i32>().unwrap() - 1) as usize;

            // println!("halves: {:?}", halves);
            // println!("quantity: {:?}", quantity);
            // println!("from: {:?}", from);
            // println!("to: {:?}", to);
            for _ in 0..quantity {
                let temp = map.stacks[from_i].pop().unwrap();
                map.stacks[to_i].push(temp);
            }

        }
    }
    // println!("Pop result of first stack: {}", map.stacks[0].pop().unwrap());
    println!("Map result: {:?}", map);
    for stack in map.stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    // println!("Score: {}", acc);
}
