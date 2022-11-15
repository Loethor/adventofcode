/* Day 07

Summary of the puzzle:

A)  
B)  

                    Processed           Processed
Example input       Part A              Part B



Example output


*/

pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 336040);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 94813675);
}

fn solve_part1(input: &Vec<String>) -> i32 {
    
    let mut crab_positions:Vec<i32> = Vec::new();

    for element in input[0].split(","){
        crab_positions.push(element.parse().expect("A number is expected."));
    }

    let max_crab = crab_positions.iter().max().unwrap();
    let min_crab = crab_positions.iter().min().unwrap();

    let mut result = i32::MAX;

    let mut sum = 0;
    for i in *min_crab..*max_crab{
        for crab in &crab_positions{
            sum += (i-crab).abs();
        }
        if result > sum {result=sum;}
        sum = 0;
    }
    result
}


fn solve_part2(input: &Vec<String>) -> i32 {
    
    let mut crab_positions:Vec<i32> = Vec::new();

    for element in input[0].split(","){
        crab_positions.push(element.parse().expect("A number is expected."));
    }

    let max_crab = crab_positions.iter().max().unwrap();
    let min_crab = crab_positions.iter().min().unwrap();

    let mut result = i32::MAX;
    let mut sum = 0;
    for i in *min_crab..*max_crab{
        for crab in &crab_positions{
            sum += summation((i-crab).abs());
        }
        if result > sum {result=sum;}
        sum = 0;
    }
    result
}

fn summation(n:i32)->i32{
    let mut sum = 0;
    for i in 0..=n{
        sum +=i;
    }
    sum
}
