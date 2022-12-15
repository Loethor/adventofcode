use std::{collections::HashMap};

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 7553);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2758);
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

fn reverse_columns_of_boxes(columns_of_boxes: &mut HashMap<usize, Vec<char>>) {
    for key in columns_of_boxes.clone().keys(){
        columns_of_boxes.get_mut(key).unwrap().reverse();
    }
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
    fn test_add_box_to_the_column() {

        // Test that the function adds characters to the correct column of the HashMap
        let line1 = String::from("abcdef");
        let line2 = String::from("ghijkl");
        let mut columns_of_boxes: HashMap<usize, Vec<char>> = HashMap::new();
        let mut expected_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['b', 'h']);
        let vec2 = Vec::from(['f', 'l']);
        expected_columns.entry(1).or_insert(vec1);
        expected_columns.entry(2).or_insert(vec2);

        add_box_to_the_column(&line1, &mut columns_of_boxes);
        add_box_to_the_column(&line2, &mut columns_of_boxes);

        assert_eq!(columns_of_boxes, expected_columns);

        // Test that the function ignores non-alphabetic characters
        let mut columns_of_boxes = HashMap::new();
        let expected_columns = HashMap::new();
        let line = String::from("1234!@#%^&*()_+-=[]{}\\|;:'\"<>,.?/~`");

        add_box_to_the_column(&line, &mut columns_of_boxes);

        assert_eq!(columns_of_boxes, expected_columns);
    }

    #[test]
    fn test_reverse_column() {

        // Test that the function adds characters to the correct column of the HashMap
        let mut columns_of_boxes: HashMap<usize, Vec<char>> = HashMap::new();
        let mut expected_columns: HashMap<usize, Vec<char>> = HashMap::new();
        let vec1 = Vec::from(['b', 'h']);
        let vec2 = Vec::from(['h', 'b']);
        columns_of_boxes.entry(1).or_insert(vec1);
        expected_columns.entry(1).or_insert(vec2);

        reverse_columns_of_boxes(&mut columns_of_boxes);

        assert_eq!(columns_of_boxes, expected_columns);
    }

    #[test]
    fn test_obtain_instructions() {
        let mut instructions = Vec::new();
        let line = String::from("1 2 3 4");

        obtain_instructions(&line, &mut instructions);

        assert_eq!(instructions, [[1, 2, 3, 4]]);


        let mut instructions = Vec::new();
        let line = String::from("1 2 3a 4b");

        obtain_instructions(&line, &mut instructions);

        assert_eq!(instructions, [[1, 2]]);
    }


}