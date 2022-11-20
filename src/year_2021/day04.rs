/* Day 04

Summary of the puzzle:

A)  
B)  

                    
Example input
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7           

*/


use std::cmp::Ordering;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 63552);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 9020);
}

fn solve_part1(input: &Vec<String>) -> i32 {
    let mut selected_numbers:Vec<i32> = Vec::new();
    parse_input(input, &mut selected_numbers);
    let mut list_of_boards:Vec<Board> = Vec::new();
    initialize_boards(input, &mut list_of_boards);
    populate_boards_rows_from_input(input, &mut list_of_boards);
    populate_boards_columns(&mut list_of_boards);
    check_for_winner_board(selected_numbers, list_of_boards)
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let mut selected_numbers:Vec<i32> = Vec::new();
    parse_input(input, &mut selected_numbers);
    let mut list_of_boards:Vec<Board> = Vec::new();
    initialize_boards(input, &mut list_of_boards);
    populate_boards_rows_from_input(input, &mut list_of_boards);
    populate_boards_columns(&mut list_of_boards);
    check_for_last_winner_board(list_of_boards, selected_numbers)
}

struct Board{
    rows:Vec<Row>,
    columns:Vec<Column>,
    board_id:i32,
}

impl Board{
    fn check_if_number_produces_bingo(&mut self, number: i32) -> bool{
        for row in &mut self.rows{
            for element in &mut row.elements{
                if number == element.value{element.got_drawn();}
            }
        }

        for column in &mut self.columns{
            for element in &mut column.elements{
                if number == element.value{element.got_drawn();}
            }
        }
        self.check_for_bingo()

    }

    fn calculate_sum_of_not_drawn_numbers(&self) -> i32{
        let mut sum = 0;
        for row in &self.rows{
            for element in &row.elements{
                if !element.is_drawn(){
                    sum += element.value;
                }
            }
        }
        sum
    }

    fn check_for_bingo(&self) -> bool{
        self.rows.iter().any(|row| row.check_for_bingo()) ||
        self.columns.iter().any(|col| col.check_for_bingo())
    }

    fn obtains_board_columns_from_rows(&mut self){
        for _ in 0..5{
            self.columns.push(Column{elements: Vec::new()})
        }
        for row in &self.rows{
            for i in 0..5{
                self.columns[i].add_element(row.elements[i].clone())
            }
        }
    }

    // debug function
    #[allow(dead_code)]
    fn print_board(&self){
        for row in &self.rows{
            for element in &row.elements{
                print!("{} ",element.value);
            }
            print!("\n");
        }
    }

    // debug function
    #[allow(dead_code)]
    fn print_by_columns(&self){
        for column in &self.columns{
            for element in &column.elements{
                print!("{} ",element.value);
            }
            print!("\n");
        }
    }

    // debug function
    #[allow(dead_code)]
    fn print_state(&self){
        for row in &self.rows{
            for element in &row.elements{
                print!("{} ",element.drawn);
            }
            print!("\n");
        }
    }
}

struct Row {
    elements:Vec<Number>,
}

impl Row {
    fn add_element(&mut self, number: Number){
        match self.elements.len() < 5 {
            true => self.elements.push(number),
            false => panic!("Board can only have 5 elements per row"),
        }
    }
    fn check_for_bingo(&self) -> bool{
        return self.elements.iter().all(|x|x.drawn);
    }
}

struct Column {
    elements:Vec<Number>,
}

impl Column {
    fn add_element(&mut self, number: Number){
        match self.elements.len().cmp(&5) {
            Ordering::Less => self.elements.push(number),
            _ => panic!("Board can only have 5 elements per column"),
        }
    }
    fn check_for_bingo(&self) -> bool{
        return self.elements.iter().all(|x|x.drawn);
    }
}

#[derive(Clone)]
struct Number{
    value:i32,
    drawn:bool,
}

impl Number {
    fn got_drawn(&mut self){
        self.drawn = true;
    }
    fn is_drawn(&self) -> bool{
        self.drawn
    }
}

fn parse_input(input: &Vec<String>, selected_numbers: &mut Vec<i32>) {
    for number in input[0].split(','){
        selected_numbers.push(number.parse().expect("Character is not numeric."));
    }
}

fn initialize_boards(input: &Vec<String>, list_of_boards: &mut Vec<Board>) {
    let mut board_id = 0;
    for line in &input[1..]{
        if line.len() == 0 {
            list_of_boards.push(Board { columns: Vec::new(), rows: Vec::new(), board_id});
            board_id += 1;
        }
    }
}

fn populate_boards_rows_from_input(input: &Vec<String>, list_of_boards: &mut Vec<Board>) {
    let mut current_board = 0;
    for line in &input[1..]{
        match line.len().cmp(&0) {
            Ordering::Equal => continue,
            Ordering::Greater => {
                add_rows_to_board(line, list_of_boards, &mut current_board);
            },
            Ordering::Less => panic!("Line size cannot be negative."),   
        }
    }
}

fn add_rows_to_board(line: &String, list_of_boards: &mut Vec<Board>, current_board: &mut usize) {
    let mut new_row = Row{elements: Vec::new()};
    for character in line.split(" "){
        if character.is_empty(){continue;} // single digit numbers have a space in front of them that must be skipped.
        new_row.add_element(Number{value: character.parse().expect("Expected a number."), drawn: false});
    }
    list_of_boards[*current_board].rows.push(new_row);
    if list_of_boards[*current_board].rows.len() == 5 {*current_board +=1}
}

fn populate_boards_columns(list_of_boards: &mut Vec<Board>) {
    for board in list_of_boards{
        board.obtains_board_columns_from_rows();
    }
}

fn check_for_winner_board(selected_numbers: Vec<i32>, mut list_of_boards: Vec<Board>) -> i32 {
    let mut board_points = 0;
    let mut winner_number = 0;
    'number_loop: for number in selected_numbers{
        for board in &mut list_of_boards{
            if board.check_if_number_produces_bingo(number){
                winner_number = number;
                board_points = board.calculate_sum_of_not_drawn_numbers();
                break 'number_loop;
            }
        }
    }
    board_points * winner_number
}

fn check_for_last_winner_board(mut list_of_boards: Vec<Board>, selected_numbers: Vec<i32>) -> i32 {
    let number_of_boards = list_of_boards.len();
    let mut board_points = 0;
    let mut winner_number = 0;
    let mut list_of_winners:Vec<i32> = Vec::new();
    'number_loop: for number in selected_numbers{
        for board in &mut list_of_boards{
            if list_of_winners.contains(&board.board_id){continue;} 
            if board.check_if_number_produces_bingo(number){
                if !list_of_winners.contains(&board.board_id){list_of_winners.push(board.board_id)}
                if list_of_winners.len() == number_of_boards{
                    winner_number = number;
                    board_points = board.calculate_sum_of_not_drawn_numbers();
                    break 'number_loop;
                }
            }
        }
    }
    board_points * winner_number
}