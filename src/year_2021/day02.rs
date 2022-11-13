/* Day 02

Summary of the puzzles:

A)  Read the input and obtain two numbers, depth and px. One is the summation of every number with the tag 
    "forward". The other  number is obtain from the numbers with the tags "up" and "down". "Down" increases it,
    "up" decreases it. The answer is depth * px
B)  There is a new number: aim. "up" decreases aim while "down" increases it. "forward" x increases  
    px by x and your depth by aim * x. The answer is depth * px

                    Processed           Processed
Example input       Part A              Part B
forward 5           px = 5              px = 5; depth = 5 * aim = 5 * 0
down 5              depth = 5           aim = 5
forward 8           px = 13 = 5 + 8     px = 13; depth = 8 * 5 = 40
up 3                depth = 2 = 5 - 3   aim = 2 = 5 - 3
down 8              depth = 10 = 2 + 8  aim = 10 = 2 + 8
forward 2           px = 15 = 13 + 2    px = 15; depth = 40 + 2 * 10 = 60

Example output
                    150 = 15 * 10       900 = 60 * 10    

*/

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1660158);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1604592846);
}

fn solve_part1(input: &Vec<String>) -> i64 {
    let mut px: i64 = 0;
    let mut depth: i64 = 0;
    for line in input.iter(){
        let mut world_iterable =  line.split_whitespace();
        let world = world_iterable.next().unwrap();
        let number: i64 = world_iterable.next().unwrap().parse().unwrap();
        match world{
            "forward" => px += number,
            "up" => depth -= number,
            "down" => depth += number,
            _ => panic!("Word not recognized")
        } 
    }
    px * depth
}

fn solve_part2(input: &Vec<String>) -> i64 {
    let mut aim: i64 = 0;
    let mut px: i64 = 0;
    let mut depth: i64 = 0;
    for line in input.iter(){
        let mut world_iterable =  line.split_whitespace();
        let world = world_iterable.next().unwrap();
        let number:i64 = world_iterable.next().unwrap().parse().unwrap();
        match world{
            "forward" =>
            {
                px += number; 
                depth += number*aim
            },
            "up" => aim -= number,
            "down" => aim += number,
            _ => panic!("Word not recognized")
        } 
    }
    px * depth
}