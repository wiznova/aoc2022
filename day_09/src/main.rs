use std::collections::HashSet;
use std::fs;

static LOG: bool = false;
static DIRECTIONS: [&str; 8] = ["U", "D", "L", "R", "UR", "UL", "DR", "DL"];
static WIDTH: i64 = 26;
static HEIGHT: i64 = 21;
static SX: i64 = 11;
static SY: i64 = 5;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    fn move_closer(&mut self, h: Point) {
        if dist(*self, h) > 1.5 {
            for mv in self.possible_moves() {
                if dist(mv, h) == 1.0 {
                    self.x = mv.x;
                    self.y = mv.y;
                    return;
                }
            }
            for mv in self.possible_moves() {
                if dist(mv, h) < 1.5 {
                    self.x = mv.x;
                    self.y = mv.y;
                    return;
                }
            }
        }
    }

    fn move_one(&mut self, dir: &str) -> Point {
        match dir {
            "U" => self.y += 1,
            "R" => self.x += 1,
            "D" => self.y -= 1,
            "L" => self.x -= 1,
            "UR" => {
                self.move_one("U");
                self.move_one("R");
            }
            "UL" => {
                self.move_one("U");
                self.move_one("L");
            }
            "DR" => {
                self.move_one("D");
                self.move_one("R");
            }
            "DL" => {
                self.move_one("D");
                self.move_one("L");
            }
            _ => (),
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

fn print_grid_ch1(h: Point, t: Point) {
    for i in (0..HEIGHT).rev() {
        for j in 0..WIDTH {
            let p = Point { x: j, y: i };
            if p == h {
                print!("H");
            } else if p == t {
                print!("T");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn print_grid_ch2(rope: &Vec<Point>) {
    for i in (0..HEIGHT).rev() {
        for j in 0..WIDTH {
            let cursor = Point { x: j, y: i };
            let mut matched = false;
            for (k, p) in rope.iter().enumerate() {
                if *p == cursor {
                    matched = true;
                    match k {
                        0 => print!("H"),
                        1..=9 => print!("{k}"),
                        _ => (),
                    }
                    break;
                }
            }
            if !matched {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn update_rope(rope: &mut Vec<Point>) {
    for i in 0..rope.len() - 1 {
        let h = rope[i];
        rope[i + 1].move_closer(h);
    }
}

fn main() {
    let file_path = "./input.txt";
    // let file_path = "./test_input.txt";
    // let file_path = "./test_input_2.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    let mut rope: Vec<Point> = Vec::new();
    for _ in 0..10 {
        rope.push(Point::new(SX, SY));
    }

    let mut t_locations = HashSet::new();
    t_locations.insert(rope[rope.len() - 1]);

    print_grid_ch2(&rope);

    for s in &split {
        let dir = &s[0..1];
        let val = s[2..s.len()].parse::<u64>().unwrap();

        for _ in 0..val {
            rope[0].move_one(dir);
            update_rope(&mut rope);
            t_locations.insert(rope[rope.len() - 1]);
        }
        if LOG {
            println!("{} {}", dir, val);
            print_grid_ch2(&rope);
        }
    }
    println!("ch2: {}", t_locations.len());

    println!("{:?}", rope);
    println!("{:?}", rope.len());

    {
        let mut h = Point { x: 0, y: 0 };
        let mut t = Point { x: 0, y: 0 };

        let mut t_locations = HashSet::new();
        t_locations.insert(t);

        print_grid_ch1(h, t);

        for s in split {
            let dir = &s[0..1];
            let val = s[2..s.len()].parse::<u64>().unwrap();

            if LOG {
                println!("{} {}", dir, val);
                print_grid_ch1(h, t);
            }
            for _ in 0..val {
                h.move_one(dir);
                t.move_closer(h);

                if LOG {
                    print_grid_ch1(h, t);
                }
                t_locations.insert(t);
            }
        }
        println!("ch1: {}", t_locations.len());
    }
}
