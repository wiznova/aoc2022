use std::fs;

#[derive(Debug)]
enum Choice {
    Rock(u8),
    Paper(u8),
    Scissors(u8),
}

#[derive(Debug)]
enum State {
    Lose(u8),
    Tie(u8),
    Win(u8),
}

fn to_choice(letter: char) -> Choice {
    match letter {
        'A' => Choice::Rock(0),
        'B' => Choice::Paper(0),
        'C' => Choice::Scissors(0),
        'X' => Choice::Rock(1),
        'Y' => Choice::Paper(2),
        'Z' => Choice::Scissors(3),
        _ => todo!(),
    }
}

fn to_state(opponent: Choice, me: Choice) -> State {
    match (me, opponent) {
        (Choice::Rock(_), Choice::Paper(_)) => State::Lose(0),
        (Choice::Paper(_), Choice::Scissors(_)) => State::Lose(0),
        (Choice::Scissors(_), Choice::Rock(_)) => State::Lose(0),
        (Choice::Rock(_), Choice::Rock(_)) => State::Tie(3),
        (Choice::Paper(_), Choice::Paper(_)) => State::Tie(3),
        (Choice::Scissors(_), Choice::Scissors(_)) => State::Tie(3),
        (Choice::Paper(_), Choice::Rock(_)) => State::Win(6),
        (Choice::Scissors(_), Choice::Paper(_)) => State::Win(6),
        (Choice::Rock(_), Choice::Scissors(_)) => State::Win(6),
        _ => todo!(),
    }
}

fn main() {
    
    let file_path = "./2_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();

    for s in split {
        break
        // println!("{}", s);
    }
    let opponent = Choice::Rock(0);
    let me = Choice::Scissors(2);
    println!("State: {:?}", to_state(opponent, me));


}
