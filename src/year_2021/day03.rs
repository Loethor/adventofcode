/* Day 03

Summary of the puzzle:

A)  We require two numbers: life support rating (lsr) and oxygen generator rating (ogr).
    For the lsr|ogr read the first bit (left most) for each number. If the mayority is a 1|0, the 
    first bit of the solution is a 1|0, else a 0|1. These will produce binary numbers. Obtain their decimal
    number and multiply them both.
B)  In this case, to obtain lsr|ogr we check the first bit and keep the numbers that start with the
    number of the mayority|minority. From that list of numbers we do the same for the second bit. Then the thrid.
    This continues until one number is alone in the list. That is your lsr|ogr. Obtain their decimal
    number and multiply them both.

                    Processed           Processed
Example input       Part A              Part B
00100               00100               00100               
11110               11110               11110               
10110               10110               10110               
10111               10111               10111               
10101               10101               10101               
01111               01111               01111               
00111               00111               00111               
11100               11100               11100               
10000               10000               10000               
11001               11001               11001               
00010               00010               00010               
01010               01010               01010               

                    1                   Too complex to represent :)
                     0
                      1
                       1
                        0

                    0
                     1
                      0
                       0 
                        1

                    in dec
                    22
                        9


Example output
                    198 = 22 * 9        230 = 23 * 10

*/

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1025636);

    let part2 = solve_part2b(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 793873);
}

fn solve_part1(input: &Vec<String>) -> i32 {

    let number_of_elements_per_row = input[0].len();
    let mut number_of_ones_per_column = vec![0; number_of_elements_per_row];
    let mut number_of_zeros_per_column = vec![0; number_of_elements_per_row];

    for line in input.iter(){
        for (i, character) in line.chars().enumerate(){
            match character{
                '1' => number_of_ones_per_column[i] += 1,
                '0' => number_of_zeros_per_column[i] += 1,
                 _ => panic!("Wrong input"),
            }
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate =  String::new();

    for i in 0..number_of_elements_per_row{
        match number_of_ones_per_column[i] > number_of_zeros_per_column[i] {
            true => 
            {
                gamma_rate  += &String::from("1");
                epsilon_rate  += &String::from("0");
            },
            false =>
            {
                gamma_rate  += &String::from("0");
                epsilon_rate  += &String::from("1");
            }  
        }
    }
    i32::from_str_radix(&gamma_rate, 2).unwrap() *  i32::from_str_radix(&epsilon_rate, 2).unwrap()
}

fn solve_part2(input: &Vec<String>) -> i32{
    
    let copy = input.clone();
    let gamma_rate = _recursive_search(copy, 0, true);
    
    let copy = input.clone();
    let epsilon_rate = _recursive_search(copy, 0, false);

    let mut gamma_rate_binary = String::new();
    let mut epsilon_rate_binary = String::new();

    for i in 0..gamma_rate.len(){

        gamma_rate_binary += &gamma_rate[i];
        epsilon_rate_binary += &epsilon_rate[i];
    }

    return  i32::from_str_radix(&gamma_rate_binary, 2).unwrap() * i32::from_str_radix(&epsilon_rate_binary, 2).unwrap() ;
}

fn _recursive_search(input:Vec<String>, j:usize, flag:bool) -> Vec<String>{

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
            match character{
                '1' => positive_lines.push(line.to_string()),
                '0' => negative_lines.push(line.to_string()),
                 _ => panic!("Wrong input"),
            }
        }
    }

    match  positive_lines.len() >= negative_lines.len()  {
        true => match flag {
            true => _recursive_search(positive_lines, j+1, true),
            false => _recursive_search(negative_lines, j+1, false),
        }
        false => match flag {
            true => _recursive_search(negative_lines, j+1, true),
            false => _recursive_search(positive_lines, j+1, false),

        }
    }
}

