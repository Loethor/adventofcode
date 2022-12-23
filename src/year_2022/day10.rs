// This file is just a template for any day solution.
use std::time::{Instant};

pub fn run(input: Vec<String>) {
    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 13740);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

    let start = Instant::now();
    let _part2 = solve_part2(&input);
    // assert_eq!(part2, ZUPRFECL);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

}

fn solve_part1(input: &Vec<String>) -> i32 {
    let list_of_commands = obtain_input(input);
    let mut history:Vec<i32> = Vec::new();

    execute_commands(list_of_commands, &mut history);
    let solution = obtain_solution(history);

    solution
}

fn obtain_solution(history: Vec<i32>) -> i32 {
    let mut solution = 0;
    for element in [20, 60, 100, 140, 180, 220].iter(){
        solution += element * history[*element as usize - 1];
    }
    println!("{}", solution);
    solution
}

fn execute_commands(list_of_commands: Vec<(Command, i32)>, history: &mut Vec<i32>) {
    let mut register = 1;
    for (command, value) in list_of_commands.iter(){
        match command {
            Command::Addx => {
                history.push(register);
                history.push(register);
                register+=value;
            },
            Command::Noop => {
                history.push(register);
            },            
        }
    }
}

fn obtain_input(input: &Vec<String>) -> Vec<(Command, i32)> {
    use Command::*;
    let mut list_of_commands:Vec<(Command,i32)> = Vec::new();
    for line in input.iter(){
        if line.starts_with("addx"){
            let (_, num) = line.split_once(" ").unwrap();
            let num:i32 = num.parse().unwrap();
            list_of_commands.push((Addx, num));
        } else {
            list_of_commands.push((Noop, 0));
        }
    }
    list_of_commands
}

fn solve_part2(input: &Vec<String>) ->i32 {
    let list_of_commands = obtain_input(input);
    let mut history:Vec<i32> = Vec::new();

    execute_commands(list_of_commands, &mut history);


    for i in 0..240 {
        if i%40 == 0 {
            println!("");
        }
        let range = [history[i]-1, history[i], history[i]+1];
        if range.contains(&((i % 40) as i32)){
            print!("#");
        } else {
            print!(".")
        }
    }
    println!("");
    0
}

#[derive(Debug, PartialEq)]
enum Command {
    Addx,
    Noop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        // Arrange

        // Act

        // Assert
    }

    #[test]
    fn test_obtain_input() {
        // Arrange
        let test_input:Vec<String> = Vec::from(["addx 15".to_string(), "noop".to_string()]);

        // Act
        let test_commands = obtain_input(&test_input);

        // Assert
        assert_eq!(test_commands[0],(Command::Addx, 15));
        assert_eq!(test_commands[1],(Command::Noop, 0));
    }
}