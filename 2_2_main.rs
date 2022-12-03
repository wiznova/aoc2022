use std::fs;

#[derive(Debug, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum State {
    Lose,
    Tie,
    Win,
}

fn to_choice(letter: char) -> Choice {
    match letter {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => todo!(),
    }
}

fn derive_choice(p1: Choice, state: State) -> Choice {
    match (p1, state) {
        (Choice::Rock, State::Lose) => Choice::Scissors,
        (Choice::Rock, State::Tie) => Choice::Rock,
        (Choice::Rock, State::Win) => Choice::Paper,
        (Choice::Paper, State::Lose) => Choice::Rock,
        (Choice::Paper, State::Tie) => Choice::Paper,
        (Choice::Paper, State::Win) => Choice::Scissors,
        (Choice::Scissors, State::Lose) => Choice::Paper,
        (Choice::Scissors, State::Tie) => Choice::Scissors,
        (Choice::Scissors, State::Win) => Choice::Rock,
    }
}
fn state_from_char(ch: char) -> State{
    match ch {
        'X' => State::Lose,
        'Y' => State::Tie,
        'Z' => State::Win,
        _ => todo!(),
    }
}


fn to_state(opponent: Choice, me: Choice) -> State {
    match (me, opponent) {
        (Choice::Rock, Choice::Paper) => State::Lose,
        (Choice::Paper, Choice::Scissors) => State::Lose,
        (Choice::Scissors, Choice::Rock) => State::Lose,
        (Choice::Rock, Choice::Rock) => State::Tie,
        (Choice::Paper, Choice::Paper) => State::Tie,
        (Choice::Scissors, Choice::Scissors) => State::Tie,
        (Choice::Paper, Choice::Rock) => State::Win,
        (Choice::Scissors, Choice::Paper) => State::Win,
        (Choice::Rock, Choice::Scissors) => State::Win,
    }
}

fn state_to_value(state: State) -> u8 {
    match state {
        State::Lose => 0,
        State::Tie => 3,
        State::Win => 6,
    }
}
fn choice_to_value(choice: Choice) -> u8 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn calculate_score(my_choice: Choice, state: State) -> u8 {
    choice_to_value(my_choice) + state_to_value(state)
}

fn main() {
    
    let file_path = "./2_input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut acc: i32 = 0;

    for s in split {
        if s == "" {
            break
        }
        let char_vec: Vec<char> = s.chars().collect();
        let opponent = to_choice(char_vec[0]);
        let state = state_from_char(char_vec[char_vec.len() - 1]);
        let me = derive_choice(opponent, state);
        // println!("state-{} {:?}", calculate_score(me, state), state);
        acc += calculate_score(me, state) as i32;
    }
    println!("Score: {}", acc);


}
