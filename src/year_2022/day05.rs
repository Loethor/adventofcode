use std::{collections::HashMap};



pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 7553);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2758);
}

fn solve_part1(input: &Vec<String>) -> u32 {

    let mut columns_of_boxes:HashMap<i32,Vec<char>> = HashMap::new();
    let mut how_many:Vec<i32> = Vec::new();
    let mut from:Vec<i32> = Vec::new();
    let mut to:Vec<i32> = Vec::new();
    let mut instructions: Vec<Vec<i32>> = Vec::new();
    for line in input.iter(){
        if line.contains("["){

            for (i, letter) in line.chars().enumerate(){
                match i {
                    // TODO make general
                    _ => if i%4 == 1 {
                        if letter != ' ' {
                            columns_of_boxes.entry((i / 4 + 1) as i32).or_insert(vec![]).push(letter);
                        }
                    }
                    _ => continue,
                }
    
            }
        } else if line.contains("move") {
            // let mut parts = line.split_whitespace().map(|s|s.parse::<i32>());
            let mut instruction = vec![];
            for element in  line.split_whitespace().map(|s|s.parse::<i32>()){
                match element {
                    Ok(a) => instruction.push(a),
                    Err(_) => {},   
                }
            }
            instructions.push(instruction);
            

        } else if line.len() == 0 {
            continue;
        } else {
            continue;
        }
        
        
        
    }
    for key in columns_of_boxes.clone().keys(){
        columns_of_boxes.get_mut(key).unwrap().reverse();
    }



    for instruction in instructions{

        for i in 0..instruction[0]{
            if let Some(item) = columns_of_boxes.get_mut(&instruction[1]).unwrap().pop(){
                columns_of_boxes.get_mut(&instruction[2]).unwrap().push(item);
            }
        }
    }


    let solution = String::from("");

    for i in 1..=columns_of_boxes.len() as i32{
        println!("{:?}", columns_of_boxes.get_mut(&i).unwrap().last())
    }






    
    0
}

fn solve_part2(input: &Vec<String>) -> u32 {

    let mut columns_of_boxes:HashMap<i32,Vec<char>> = HashMap::new();
    let mut how_many:Vec<i32> = Vec::new();
    let mut from:Vec<i32> = Vec::new();
    let mut to:Vec<i32> = Vec::new();
    let mut instructions: Vec<Vec<i32>> = Vec::new();
    for line in input.iter(){
        if line.contains("["){

            for (i, letter) in line.chars().enumerate(){
                match i {
                    // TODO make general
                    _ => if i%4 == 1 {
                        if letter != ' ' {
                            columns_of_boxes.entry((i / 4 + 1) as i32).or_insert(vec![]).push(letter);
                        }
                    }
                    _ => continue,
                }
    
            }
        } else if line.contains("move") {
            // let mut parts = line.split_whitespace().map(|s|s.parse::<i32>());
            let mut instruction = vec![];
            for element in  line.split_whitespace().map(|s|s.parse::<i32>()){
                match element {
                    Ok(a) => instruction.push(a),
                    Err(_) => {},   
                }
            }

            instructions.push(instruction);
            

        } else if line.len() == 0 {
            continue;
        } else {
            continue;
        }
    }

    for key in columns_of_boxes.clone().keys(){
        columns_of_boxes.get_mut(key).unwrap().reverse();
    }



    for instruction in instructions{
        let mut boxes:Vec<char> = vec![];
        for i in 0..instruction[0]{

            if let Some(item) = columns_of_boxes.get_mut(&instruction[1]).unwrap().pop(){
                boxes.push(item);
            }
        }
        boxes.reverse();
        columns_of_boxes.get_mut(&instruction[2]).unwrap().append(&mut boxes);
    }
    let solution = String::from("");
    for i in 1..=columns_of_boxes.len() as i32{
        println!("{:?}", columns_of_boxes.get_mut(&i).unwrap().last())
    }
    0
}
