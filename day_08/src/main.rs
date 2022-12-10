use std::fs;

struct Forest<'a> {
    ar: Vec<&'a str>,
    max_i: usize,
}

impl Forest<'_> {
    fn ch(&self, row: usize, col: usize) -> char {
        self.ar[row].chars().nth(col).unwrap()
    }
    fn visible_up(&self, row: usize, col: usize) -> bool {
        let height = self.ch(row, col);
        if row == 0 {
            return true;
        }
        for i in 0..row {
            if self.ch(i, col) >= height {
                return false;
            }
        }
        true
    }
    fn visible_down(&self, row: usize, col: usize) -> bool {
        let height = self.ch(row, col);
        if row == self.max_i {
            return true;
        }
        for i in row + 1..self.max_i + 1 {
            if self.ch(i, col) >= height {
                return false;
            }
        }
        true
    }
    fn visible_left(&self, row: usize, col: usize) -> bool {
        let height = self.ch(row, col);
        if col == 0 {
            return true;
        }
        for i in 0..col {
            if self.ch(row, i) >= height {
                return false;
            }
        }
        true
    }
    fn visible_right(&self, row: usize, col: usize) -> bool {
        let height = self.ch(row, col);
        if col == self.max_i {
            return true;
        }
        for i in col + 1..self.max_i + 1 {
            if self.ch(row, i) >= height {
                return false;
            }
        }
        true
    }
    fn visible(&self, row: usize, col: usize) -> bool {
        return self.visible_down(row, col)
            || self.visible_up(row, col)
            || self.visible_left(row, col)
            || self.visible_right(row, col);
    }

    fn count_left(&self, row: usize, col: usize) -> u64 {
        let height = self.ch(row, col);
        let mut co = 0;
        if col == 0 {
            return 0;
        }
        for i in (0..col).rev() {
            co += 1;
            if self.ch(row, i) >= height {
                break;
            }
        }
        co
    }
    fn count_right(&self, row: usize, col: usize) -> u64 {
        let height = self.ch(row, col);
        let mut co = 0;
        if col == self.max_i {
            return 0;
        }
        for i in col + 1..self.max_i + 1 {
            co += 1;
            if self.ch(row, i) >= height {
                break;
            }
        }
        co
    }
    fn count_down(&self, row: usize, col: usize) -> u64 {
        let height = self.ch(row, col);
        let mut co = 0;
        if row == self.max_i {
            return 0;
        }
        for i in row + 1..self.max_i + 1 {
            co += 1;
            if self.ch(i, col) >= height {
                break;
            }
        }
        co
    }
    fn count_up(&self, row: usize, col: usize) -> u64 {
        let height = self.ch(row, col);
        let mut co = 0;
        if row == 0 {
            return 0;
        }
        for i in (0..row).rev() {
            co += 1;
            if self.ch(i, col) >= height {
                break;
            }
        }
        co
    }
    fn scenic_score(&self, row: usize, col: usize) -> u64 {
        return self.count_down(row, col)
            * self.count_up(row, col)
            * self.count_left(row, col)
            * self.count_right(row, col);
    }
}

fn main() {
    let file_path = "./input.txt";
    // let file_path = "./test_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let s_len = split.len();

    let f = Forest {
        ar: split,
        max_i: s_len - 1,
    };

    // challenge 1
    {
        let mut total = 0u64;

        for row in 0..s_len {
            for col in 0..s_len {
                if f.visible(row, col) {
                    total += 1;
                }
            }
        }
        println!("ch1: {total}");
    }

    // challenge 2
    {
        let mut max_score = 0u64;
        for row in 0..s_len {
            for col in 0..s_len {
                let score =f.scenic_score(row, col); 
                if score  > max_score {
                    max_score = score;
                }
            }
        }
        println!("ch2: {max_score}");
    }

    // println!("{}", f.visible_right(0, 4));
    // println!("{}", f.visible_left(f.max_i, f.max_i));
    // println!("{}", f.visible_up(1, f.max_i));
    // println!("{}", f.visible_down(3, f.max_i));

    // println!("{}", ch(split[0], 0));

    // for s in split {
    //     println!("{s}");
    // }
}
