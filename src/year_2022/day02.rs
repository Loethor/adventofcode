use std::collections::HashMap;
use self::Hand::*;
use self::GameResult::*;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 15481);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 16862);
}

fn solve_part1(input: &Vec<String>) -> i32 {

    let letter_to_hand:HashMap<&str, Hand> = HashMap::from([
        ("A", Rock),
        ("B", Paper),
        ("C", Scissors),
        ("X", Rock),
        ("Y", Paper),
        ("Z", Scissors),
    ]);

    let mut points = 0;
    for line in input.iter(){
        let (their_hand, my_hand) = line.split_once(' ').unwrap();
        let (their_hand, my_hand ) = (letter_to_hand[their_hand], letter_to_hand[my_hand]);
        points += calculate_points(my_hand, their_hand);
    }
    points
}


fn solve_part2(input: &Vec<String>) -> i32 {

    let letter_to_hand:HashMap<&str, Hand> = HashMap::from([
        ("A", Rock),
        ("B", Paper),
        ("C", Scissors),
    ]);

    let letter_to_game_result:HashMap<&str, GameResult> = HashMap::from([
        ("X", Lose),
        ("Y", Draw),
        ("Z", Win),
    ]);

    let mut points = 0;
    for line in input.iter(){
        let (their_hand, game_result) = line.split_once(' ').unwrap();
        let (their_hand, my_result) = (letter_to_hand[their_hand], letter_to_game_result[game_result]);
        match my_result {
            Win => points += calculate_points(their_hand.beats().beats(), their_hand),
            Draw => points += calculate_points(their_hand, their_hand),
            Lose => points += calculate_points(their_hand.beats(), their_hand),
        }
    }
    points
}

pub fn calculate_points(my_hand:Hand, their_hand:Hand, ) -> i32 {
    return my_hand.hands_to_points() + game(my_hand, their_hand).game_result_to_points();
}

fn game(my_hand:Hand, their_hand:Hand) -> GameResult{
    let (my_hand_beats, their_hand_beats) = (my_hand.beats(), their_hand.beats());

    match (my_hand_beats, their_hand_beats) {
        _ if my_hand_beats == their_hand => Win,
        _ if their_hand_beats == my_hand => Lose,
        _                            => Draw, 
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    fn hands_to_points(&self) -> i32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}
pub trait Beats{
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}



#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn game_result_to_points(&self) -> i32 {
        match *self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps_game() {
        assert_eq!(game(Rock, Paper), Lose);
        assert_eq!(game(Rock, Rock), Draw);
        assert_eq!(game(Rock, Scissors), Win);
        assert_eq!(game(Paper, Scissors), Lose);
        assert_eq!(game(Paper, Paper), Draw);
        assert_eq!(game(Paper, Rock), Win);
        assert_eq!(game(Scissors, Rock), Lose);
        assert_eq!(game(Scissors, Scissors), Draw);
        assert_eq!(game(Scissors, Paper), Win);
    }

    #[test]
    fn test_calculate_points() {
        assert_eq!(calculate_points(Rock, Rock), 4);
        assert_eq!(calculate_points(Rock, Paper), 1);
        assert_eq!(calculate_points(Rock, Scissors), 7);
        assert_eq!(calculate_points(Paper, Rock), 8);
        assert_eq!(calculate_points(Paper, Paper), 5);
        assert_eq!(calculate_points(Paper, Scissors), 2);
        assert_eq!(calculate_points(Scissors, Rock), 3);
        assert_eq!(calculate_points(Scissors, Paper), 9);
        assert_eq!(calculate_points(Scissors, Scissors), 6);
    }
}