pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1025636);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 1604592846);
}

fn solve_part1(input: &Vec<String>) -> i32 {

    let number_of_elements = input.iter().next().unwrap().len();
    let mut number_of_ones = vec![0; number_of_elements];
    let mut number_of_zeros = vec![0; number_of_elements];

    for line in input.iter(){
        let mut i = 0;
        for character in line.chars(){
            if character == '1'{
                number_of_ones[i] += 1;
            } else if character == '0' {
                number_of_zeros[i] += 1;
            }
            i += 1;
        }
    }


    let mut gamma_rate = vec![0; number_of_elements];
    let mut epsilon_rate = vec![0; number_of_elements];
    for i in 0..number_of_elements{
        if number_of_ones[i] > number_of_zeros[i]{
            gamma_rate[i] = 1;
            epsilon_rate[i] = 0;
        } else{
            gamma_rate[i] = 0;
            epsilon_rate[i] = 1;

        }
    }
    let mut gamma_rate_dec = 0;
    let mut epsilon_rate_dec = 0;
    for i in 0..number_of_elements{
        let operator = i32::pow(2, ((number_of_elements - 1) - i ) as u32);
        gamma_rate_dec += gamma_rate[i] * operator;
        epsilon_rate_dec += epsilon_rate[i] * operator;
    }

    return  gamma_rate_dec * epsilon_rate_dec;
}

fn solve_part2(input: &Vec<String>) -> i32{

    let number_of_elements = input.iter().next().unwrap().len();
    let mut number_of_ones = vec![0; number_of_elements];
    let mut number_of_zeros = vec![0; number_of_elements];

    let copy = input.clone();

    let final_number_pos = recursive_search(copy, 0, true);
    
    let copy = input.clone();
    let final_number_neg = recursive_search(copy, 0, false);

    println!("{}", final_number_pos[0]);
    println!("{}", final_number_neg[0]);

    
    return 0;
}

fn recursive_search(input:Vec<String>, j:usize, flag:bool) -> Vec<String>{

    if input.len() == 1 {
        return input;
    }
    let mut positive_lines:Vec<String> = Vec::new();
    let mut negative_lines:Vec<String> = Vec::new();

    for line in input.iter(){
        for (i, character) in line.chars().enumerate(){
            if j != i{
                continue;
            }
            if character == '1'{
                positive_lines.push(line.to_string());
            } else if character == '0' {
                negative_lines.push(line.to_string());
            }
        }
    }

    if positive_lines.len() >= negative_lines.len(){
        if flag {
            recursive_search(positive_lines, j+1, true)
        } else {
            recursive_search(negative_lines, j+1, false)
        }
    } else {
        if flag {
            recursive_search(negative_lines, j+1, true)
        } else {
            recursive_search(positive_lines, j+1, false)
        }
    }
}