use std::fs;

static TICKS: [i64; 6] = [20, 60, 100, 140, 180, 220];
static WIDTH: i64 = 40;

struct CPU {
    _co: i64,
    x: i64,
    _sum: i64,
}

impl CPU {
    fn tick(&mut self) {
        self._co += 1;
        self.draw_pixel();
        if TICKS.contains(&self._co) {
            self._sum += self._co * self.x;
        }
    }
    fn draw_pixel(&self) {
        let pos = self._co % WIDTH;
        let sprite_pos = if self.x >= 0 {
            self.x % WIDTH
        } else {
            WIDTH + self.x
        };
        // println!("cycle {}: {pos} {sprite_pos}", self._co);

        let diff = pos - sprite_pos;
        if diff >= 0 && diff <= 2 {
            print!("#");
        } else {
            print!(".");
        }

        if pos == 0 {
            print!("\n");
        }
    }
}

fn main() {
    let file_path = "./input.txt";
    // let file_path = "./test.txt";
    // let file_path = "./small_test.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    let mut cpu = CPU {
        _co: 0,
        x: 1,
        _sum: 0,
    };

    for s in &split {
        // start of the cycle
        cpu.tick();
        if *s == "noop" {
            continue;
        }
        let instr_split: Vec<&str> = s.split(" ").collect();
        let instr = instr_split[0];
        let param = instr_split[1].parse::<i64>().unwrap();

        // second cycle
        cpu.tick();

        // end of second cycle
        cpu.x += param;

        // end of instruction
        // println!("{} {}", instr, param);
    }
    println!("sum: {}", cpu._sum);
}
