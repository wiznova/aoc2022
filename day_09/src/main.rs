use std::fs;

static DIRECTIONS: [&str; 8] = ["U", "D", "L", "R", "UR", "UL", "DR", "DL"];

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_one(&mut self, dir: &str) -> Point {
        match dir {
            "U" => self.y += 1,
            "R" => self.x += 1,
            "D" if self.y > 0 => self.y -= 1,
            "L" if self.x > 0 => self.x -= 1,
            "UR" => {
                self.move_one("U");
                self.move_one("R");
            }
            "UL" if self.x > 0 => {
                self.move_one("U");
                self.move_one("L");
            }
            "DR" if self.y > 0 => {
                self.move_one("D");
                self.move_one("R");
            }
            "DL" if self.y > 0 && self.x > 0 => {
            self.move_one("D");
                self.move_one("L");
            }
            _ => (),
            // _ => println!("Bumped wall with: {} and {:?}", dir, self),
        }
        *self
    }
    fn possible_moves(&self) -> Vec<Point> {
        let mut moves: Vec<Point> = Vec::new();
        for dir in DIRECTIONS {
            let possible_move = self.clone().move_one(dir);
            if *self != possible_move {
                moves.push(possible_move);
            }
        }
        moves
    }
}

fn dist(a: Point, b: Point) -> f64 {
    let sum = (b.x - a.x).pow(2) as f64 + (b.y - a.y).pow(2) as f64;
    sum.sqrt()
}

fn main() {
    // let file_path = "./input.txt";
    let file_path = "./test_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    let mut h = Point { x: 0, y: 0 };
    let mut t = Point { x: 0, y: 0 };

    // println!("{:?}", h.move_one("D"));
    // println!("{:?}", h.possible_moves());

    for s in split {
        let dir = &s[0..1];
        let val = s[2..s.len()].parse::<u64>().unwrap();

        println!("{} {}", dir, val);
        println!("START HEAD: {:?}", h);
        println!("START TAIL: {:?}", t);
        for _ in 0..val {
            h.move_one(dir);
            println!("HEAD: {:?}", h);

            if dist(t, h) > 1.5 {
                for mv in t.possible_moves() {
                    if dist(mv, h) == 1.0 {
                        t = mv;
                        break;
                    }
                }
            }
            println!("TAIL: {:?}", t);

        }
    }
}
