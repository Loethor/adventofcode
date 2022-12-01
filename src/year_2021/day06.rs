/* Day 06

Summary of the puzzle:

A)  
B)  

                    Processed           Processed
Example input       Part A              Part B



Example output


*/

// // For time measurement
use std::time::{Instant};

pub fn run(input: Vec<String>) {

    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    let duration = start.elapsed();
    assert_eq!(part1, 354564);


    let start = Instant::now();
    let part2 = solve_part2(&input);
    let duration2 = start.elapsed();
    println!("part 2: {}", part2);
    assert_eq!(part2, 1609058859115);

    println!("Time elapsed in solve_part1() is: {:?}", duration);
    println!("Time elapsed in solve_part2() is: {:?}", duration2);
}

fn solve_part1(input: &Vec<String>) -> i64 {
    const DAYS:usize = 80;

    let mut fish_counter:Vec<i64> = vec![0; 9];
    
    for element in input[0].split(","){
        let id = element.parse::<usize>().expect("A number is expected.");
        fish_counter[id] +=1;
    }

    let mut fish_to_add = 0; 
    for _ in 0..DAYS{       
        for i in 0..fish_counter.len(){
            match i {
                0 => {
                        fish_to_add += fish_counter[0];
                        fish_counter[0] = fish_counter[1]
                    },
                1 => fish_counter[1] = fish_counter[2],
                2 => fish_counter[2] = fish_counter[3],
                3 => fish_counter[3] = fish_counter[4],
                4 => fish_counter[4] = fish_counter[5],
                5 => fish_counter[5] = fish_counter[6],
                6 => fish_counter[6] = fish_counter[7] + fish_to_add,
                7 => fish_counter[7] = fish_counter[8],
                8 => fish_counter[8] = fish_to_add,
                _ => panic!("Something went wrong."),
            }
        }
        fish_to_add = 0;
    }
    fish_counter.iter().sum()
}

fn solve_part2(input: &Vec<String>) -> i64 {

    const DAYS:u16 = 256;

    let mut fish_counter:Vec<i64> = vec![0; 9];
    
    for element in input[0].split(","){
        let id = element.parse::<usize>().expect("A number is expected.");
        fish_counter[id] +=1;
    }

    let mut fish_to_add = 0; 
    for _ in 0..DAYS{      
        for i in 0..fish_counter.len(){
            match i {
                0 => {
                        fish_to_add += fish_counter[0];
                        fish_counter[0] = fish_counter[1]
                    },
                1 => fish_counter[1] = fish_counter[2],
                2 => fish_counter[2] = fish_counter[3],
                3 => fish_counter[3] = fish_counter[4],
                4 => fish_counter[4] = fish_counter[5],
                5 => fish_counter[5] = fish_counter[6],
                6 => fish_counter[6] = fish_counter[7] + fish_to_add,
                7 => fish_counter[7] = fish_counter[8],
                8 => fish_counter[8] = fish_to_add,
                _ => panic!("Something went wrong."),
            }
        }
        fish_to_add = 0;
    }
    fish_counter.iter().sum()
}




// Brute force using arrays
#[allow(dead_code)]
fn solve_part1b(input: &Vec<String>) -> i32 {

    const OLD_FISH_BIRTH_RATE:i32  = 6;
    const NEW_FISH_BIRTH_RATE:i32  = 8;
    const DAYS:i32 = 80;


    let mut fish_vector:Vec<i32> = Vec::new();

    for element in input[0].split(","){
        fish_vector.push(element.parse().expect("A number is expected."));
    }

    let mut fish_to_add = 0; 

    for d in 0..DAYS{
        println!("{d}");


        
        for f_id in 0..fish_vector.len(){
            if fish_vector[f_id] > 0 {
                fish_vector[f_id] -= 1;
            } else if fish_vector[f_id] == 0 {
                fish_to_add += 1;
                fish_vector[f_id] = OLD_FISH_BIRTH_RATE;
            }
        }

        for _ in 0..fish_to_add{
            fish_vector.push(NEW_FISH_BIRTH_RATE);
        }
        fish_to_add = 0;
    }

    fish_vector.len() as i32
}

#[allow(dead_code)]
fn print_fish_array(fish_vec:&Vec<i32>){
    for fish in fish_vec{
        print!("{} ", fish);
    }
    println!("\n");
}