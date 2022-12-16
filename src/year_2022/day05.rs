use std::{collections::HashMap};

use std::time::{Duration, Instant};

pub fn run(input: Vec<String>) {

    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, "CWMTGHBDW");
    let duration = start.elapsed();

    println!("Time elapsed in part1 is: {:?}", duration);

    let start = Instant::now();
    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, "SSCGWJCRB");
    let duration = start.elapsed();

    println!("Time elapsed in part2 is: {:?}", duration);
}

fn solve_part1(input: &Vec<String>) -> String {
    let mut columns_of_boxes:HashMap<usize,Vec<char>> = HashMap::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    for line in input.iter(){
        if line.contains("["){
            add_box_to_the_column(line, &mut columns_of_boxes);
        } else if line.contains("move") {
            obtain_instructions(line, &mut instructions);
        } 
    }
    reverse_columns_of_boxes(&mut columns_of_boxes);
    apply_move_instructions_to_boxes(instructions, &mut columns_of_boxes);
    let solution = construct_solution(columns_of_boxes);
    solution
}

fn solve_part2(input: &Vec<String>) -> String {
    let mut columns_of_boxes:HashMap<usize,Vec<char>> = HashMap::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    for line in input.iter(){
        if line.contains("["){
            add_box_to_the_column(line, &mut columns_of_boxes);
        } else if line.contains("move") {
            obtain_instructions(line, &mut instructions);
        }
    }
    reverse_columns_of_boxes(&mut columns_of_boxes);
    apply_move_instructions_grabbing_multiple_boxes(instructions, &mut columns_of_boxes);
    let solution = construct_solution(columns_of_boxes);
    solution
}

fn add_box_to_the_column(line: &String, columns_of_boxes: &mut HashMap<usize, Vec<char>>) {
    for (i, letter) in line.chars().enumerate(){
        if i%4 == 1 && letter.is_alphabetic(){
            let entry_id = 1 + i/4;
            columns_of_boxes.entry(entry_id).or_insert(vec![]).push(letter);
        }
    }
}

fn obtain_instructions(line: &String, instructions: &mut Vec<Vec<usize>>) {
    let mut instruction: Vec<usize> = vec![];
    line.split_whitespace().map(|s|s.parse::<usize>()).for_each(|element| {
        if let Ok(a) = element {instruction.push(a)};
    });
    instructions.push(instruction);
}

fn reverse_columns_of_boxes(columns_of_boxes: &mut HashMap<usize, Vec<char>>) {
    for key in columns_of_boxes.clone().keys(){
        columns_of_boxes.get_mut(key).unwrap().reverse();
    }
}

fn apply_move_instructions_to_boxes(instructions: Vec<Vec<usize>>, columns_of_boxes: &mut HashMap<usize, Vec<char>>) {
    for instruction in instructions{
        for _ in 0..instruction[0]{
            if let Some(item) = columns_of_boxes.get_mut(&instruction[1]).unwrap().pop(){
                columns_of_boxes.get_mut(&instruction[2]).unwrap().push(item);
            }
        }
    }
}

fn construct_solution(mut columns_of_boxes: HashMap<usize, Vec<char>>) -> String {
    let mut solution = String::from("");
    for i in 1..=columns_of_boxes.len(){
        solution.push(*columns_of_boxes.get_mut(&i).unwrap().last().unwrap());
    }
    solution
}

fn apply_move_instructions_grabbing_multiple_boxes(instructions: Vec<Vec<usize>>, columns_of_boxes: &mut HashMap<usize, Vec<char>>) {
    for instruction in instructions{
        let mut boxes:Vec<char> = vec![];
        for _ in 0..instruction[0]{
            if let Some(item) = columns_of_boxes.get_mut(&instruction[1]).unwrap().pop(){
                boxes.push(item);
            }
        }
        boxes.reverse();
        columns_of_boxes.get_mut(&instruction[2]).unwrap().append(&mut boxes);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_box_to_the_column_good_weather() {
        // Arrage
        let mut test_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let line1 = String::from("abcdef");
        let line2 = String::from("ghijkl");
        let vec1 = Vec::from(['b', 'h']);
        let vec2 = Vec::from(['f', 'l']);
        let mut expected_columns: HashMap<usize, Vec<char>> = HashMap::new();
        expected_columns.entry(1).or_insert(vec1);
        expected_columns.entry(2).or_insert(vec2);

        // Act
        add_box_to_the_column(&line1, &mut test_columns);
        add_box_to_the_column(&line2, &mut test_columns);

        // Assert
        assert_eq!(test_columns, expected_columns);
    }

    #[test]
    fn test_add_box_to_the_column_bad_weather() {
        // Arrage
        let mut test_columns = HashMap::new();
        let line = String::from("1234!@#%^&*()_+-=[]{}\\|;:'\"<>,.?/~`");
        let expected_columns = HashMap::new();

        // Act
        add_box_to_the_column(&line, &mut test_columns);

        // Assert
        assert_eq!(test_columns, expected_columns);
    }

    #[test]
    fn test_obtain_instructions_good_weater() {
        // Arrage
        let mut instructions = Vec::new();
        let line = String::from("1 2 3 4");
        
        // Act
        obtain_instructions(&line, &mut instructions);
        
        // Assert
        assert_eq!(instructions, [[1, 2, 3, 4]]);
    }
    
    #[test]
    fn test_obtain_instructions_bad_weater() {
        // Arrage
        let mut instructions = Vec::new();
        let line = String::from("1 2 3a 4b");
        
        // Act
        obtain_instructions(&line, &mut instructions);
        
        // Assert
        assert_eq!(instructions, [[1, 2]]);
    }
    
    #[test]
    fn test_reverse_column() {
        // Arrage
        let mut test_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['b', 'h']);
        test_columns.entry(1).or_insert(vec1);
        let mut expected_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec2 = Vec::from(['h', 'b']);
        expected_columns.entry(1).or_insert(vec2);

        // Act
        reverse_columns_of_boxes(&mut test_columns);

        // Assert
        assert_eq!(test_columns, expected_columns);
    }

    #[test]
    fn test_apply_move_instructions() {
        // Arrage
        let mut test_instructions = Vec::new();
        let test_instruction1 = Vec::from([1,2,3]);
        let test_instruction2 = Vec::from([2,3,1]);
        test_instructions.push(test_instruction1);
        test_instructions.push(test_instruction2);

        let mut test_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['a', 'b', 'c']);
        let vec2 = Vec::from(['d', 'e', 'f']);
        let vec3 = Vec::from(['g', 'h', 'i']);
        test_columns.entry(1).or_insert(vec1);
        test_columns.entry(2).or_insert(vec2);
        test_columns.entry(3).or_insert(vec3);

        let mut exp_columns: HashMap<usize, Vec<char>> = HashMap::new();

        let vec1 = Vec::from(['a', 'b', 'c', 'f', 'i']);
        let vec2 = Vec::from(['d', 'e']);
        let vec3 = Vec::from(['g', 'h']);
        exp_columns.entry(1).or_insert(vec1);
        exp_columns.entry(2).or_insert(vec2);
        exp_columns.entry(3).or_insert(vec3);

        // Act
        apply_move_instructions_to_boxes(test_instructions,&mut test_columns);

        // Assert
        assert_eq!(test_columns, exp_columns);
    }

    #[test]
    fn test_apply_move_multiple_instructions() {
        // Arrage
        let mut test_instructions = Vec::new();
        let test_instruction1 = Vec::from([1,2,3]);
        let test_instruction2 = Vec::from([2,3,1]);
        test_instructions.push(test_instruction1);
        test_instructions.push(test_instruction2);

        let mut test_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['a', 'b', 'c']);
        let vec2 = Vec::from(['d', 'e', 'f']);
        let vec3 = Vec::from(['g', 'h', 'i']);
        test_columns.entry(1).or_insert(vec1);
        test_columns.entry(2).or_insert(vec2);
        test_columns.entry(3).or_insert(vec3);

        let mut exp_columns: HashMap<usize, Vec<char>> = HashMap::new();

        let vec1 = Vec::from(['a', 'b', 'c', 'i', 'f']);
        let vec2 = Vec::from(['d', 'e']);
        let vec3 = Vec::from(['g', 'h']);
        exp_columns.entry(1).or_insert(vec1);
        exp_columns.entry(2).or_insert(vec2);
        exp_columns.entry(3).or_insert(vec3);

        // Act
        apply_move_instructions_grabbing_multiple_boxes(test_instructions,&mut test_columns);

        // Assert
        assert_eq!(test_columns, exp_columns);
    }
    
    #[test]
    fn test_construct_solution() {
        // Arrage
        let mut test_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['a', 'b', 'c']);
        let vec2 = Vec::from(['d', 'e', 'f']);
        let vec3 = Vec::from(['g', 'h', 'i']);
        test_columns.entry(1).or_insert(vec1);
        test_columns.entry(2).or_insert(vec2);
        test_columns.entry(3).or_insert(vec3);

        let exp_string = String::from("cfi");

        // Act
        let test_string = construct_solution(test_columns);

        // Assert
        assert_eq!(test_string, exp_string);
    }
}