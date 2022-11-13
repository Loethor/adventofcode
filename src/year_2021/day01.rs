/* Day 01

Summary of the puzzles:

A) Reading a list of numbers number and counting how many times each number is greater then the previous.
B) Obtaining the sums of a three-number sliding window and counting how many times the sum increases.

                    Processed           Processed
Example input       Part A              Part B
199                 N/A                 607 (199+200+208)
200                 Increased           618
208                 Increased           618      
210                 Increased           617
200                 Decreased           647
207                 Increased           716
240                 Increased           769
269                 Increased           792
260                 Decreased           
263                 Increased           

Example output
                    7                   5

*/

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
    for i in 1..input.len() {
        if input[i] > input[i-1] {count += 1;}
    }
    count
}

fn solve_part2(input: &Vec<i64>) -> i64 {
    let mut count: i64 = 0;
    let mut previous_sum = input[0] + input[1] + input[2];
    for element in input[1..].windows(3){
        let sum = element[0]+element[1]+element[2];
        if sum > previous_sum{count+=1}
        previous_sum = sum;
    }
    count
}