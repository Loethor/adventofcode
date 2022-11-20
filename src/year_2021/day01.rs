/* Day 01

Summary of the puzzle:

A) Reading a list of numbers number and counting how many times each number is greater then the previous.
B) Obtaining the sums of a three-number sliding window and counting how many times the sum increases.

Example input
199
200
208
210
200
207
240
269
260
263 

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


/* Alternative solutions based in the .windows() method
    A)  For part 1 you just compare the two elements in the windows and count how many times w[1] > w[0]
    B)  For part 2 you can take  4-element windows and compare the sums w[4]+w[3]+w[2] and w[3]+w[2]+w[1]
        Since you are adding, you can cancel out w[3] and w[2] and just compare w[4] and w[1]. Then you can
        abstract it as a function and compare w[window_size - 1] and w[1]
*/ 

#[allow(dead_code)]
fn solve_part1b(input: &Vec<i64>) -> usize {
    compare_elements_in_window(input, 2)
}

#[allow(dead_code)]
fn solve_part2b(input: &Vec<i64>) -> usize {
    compare_elements_in_window(input, 4)
}

fn compare_elements_in_window(input: &Vec<i64>, windows_size:usize) -> usize {
    let count = input.windows(windows_size)
                            .filter(|&w| w[windows_size-1] > w[0])
                            .count();
    count
}
