use std::fs;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn encloses(&self, other: Range) -> bool {
        if self.start <= other.start && self.end >= other.end {
            true
        } else {
            false
        }
    }
}

fn main() {
    let file_path = "./4_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut acc: u32 = 0;

    for s in split {
        let ranges: Vec<&str> = s.split(",").collect();

        let left_range_str: Vec<&str> = ranges[0].split("-").collect();
        let left_range = Range {
            start: left_range_str[0].parse::<i32>().unwrap(),
            end: left_range_str[1].parse::<i32>().unwrap(),
        };

        let right_range_str: Vec<&str> = ranges[1].split("-").collect();
        let right_range = Range {
            start: right_range_str[0].parse::<i32>().unwrap(),
            end: right_range_str[1].parse::<i32>().unwrap(),
        };

        if left_range.encloses(right_range) || right_range.encloses(left_range) {
            acc += 1;
        }
        // println!("Line {}", s);
    }

    println!("Score: {}", acc);
}
