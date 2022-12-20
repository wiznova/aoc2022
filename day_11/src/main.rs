use std::fs;

struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    _test: Box<dyn Fn(i64) -> i64>,
    _inspected: i64,
}

impl Monkey {
    fn test(&self, item: i64) -> i64 {
        (*self._test)(item)
    }
}

fn divisible_by_test(num: i64, if_true: i64, if_false: i64) -> Box<dyn Fn(i64) -> i64> {
    Box::new(|x| if x % num == 0 {if_true} else {if_false})
}

fn main() {
    let file_path = "./input.txt";
    // let file_path = "./test.txt";
    // let file_path = "./small_test.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    let m0 = Monkey{
        items: vec![79, 98],
        operation: |x| x * 19,
        _test: divisible_by_test(23, 2, 3),
    };
    let m1 = Monkey{
        items: vec![54, 65, 75, 74],
        operation: |x| x + 6,
        _test: divisible_by_test(19, 2, 0),
    };
    let m2 = Monkey{
        items: vec![79, 60, 97],
        operation: |x| x * x,
        _test: divisible_by_test(13, 1, 3),
    };
    let m3 = Monkey{
        items: vec![74],
        operation: |x| x + 3,
        _test: divisible_by_test(17, 0, 1),
    };

    let monks = [m0, m1, m2, m3];


    for s in &split {
        println!("{s}");
    }
}
