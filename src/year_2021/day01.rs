pub fn run(input: Vec<i64>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1215);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1150);
}

fn solve_part1(input: &Vec<i64>) -> i64 {
    let mut count: i64 = 0;
    
    let mut previous_element: i64 = input[0];
    println!("N/A - no previous measurement");

    for i in 1..input.len() {
        if input[i] > previous_element {
            println!("increased");
            count += 1;
        } else {
            println!("decreased");
        }
        previous_element = input[i]
    }
    return count;
}

fn solve_part2(input: &Vec<i64>) -> i64 {
    let mut count: i64 = 0;
    
    let mut first_element = input[0];
    let mut second_element = input[1];
    let mut third_element = input[2];

    let mut previous_sum = first_element + second_element + third_element;

    for i in 3..input.len() {
        first_element = second_element;
        second_element = third_element;
        third_element = input[i];
        let sum = first_element + second_element + third_element;
        if sum > previous_sum{
            println!("increased");
            count += 1;
        } else {
            println!("decreased");
        }
        previous_sum = sum
    }
    return count;
}