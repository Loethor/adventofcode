// This file is just a template for any day solution.
use std::time::{Instant};

pub fn run(input: Vec<String>) {
    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1647);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

    let start = Instant::now();
    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2447);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

}

fn solve_part1(input: &Vec<String>) -> usize {
    for line in input.iter(){

    }
    0
}

fn solve_part2(input: &Vec<String>) -> usize {
    for line in input.iter(){
        
    }
    0
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
}