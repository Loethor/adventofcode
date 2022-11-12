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

    let header = &input[0];
    let mut numbers_drawn:Vec<i32> = Vec::new();
    for number in header.split(','){
        numbers_drawn.push(number.parse().expect("Couldn't parse!"));
    }

    let mut list_of_boards:Vec<Board> = Vec::new();

    let mut board_id = 0;
    for line in &input[1..]{
        if line.len() == 0 {
            list_of_boards.push(Board { columns: Vec::new(), rows: Vec::new(), id: board_id});
            board_id += 1;
        }
    }

    let mut current_board = 0;
    for line in &input[1..]{
        match line.len().cmp(&0) {
            Ordering::Equal => continue,
            Ordering::Greater => {
                let mut new_row = Row{elements: Vec::new()};
                for character in line.split(" "){
                    if character.is_empty(){continue;}
                    let new_number = Number{value: character.trim().parse().expect("Expected a number."), drawn: false};
                    new_row.add_element(new_number);
                }
                list_of_boards[current_board].rows.push(new_row);
                if list_of_boards[current_board].rows.len() == 5 {current_board +=1}
            },
            Ordering::Less => panic!("Lines cannot be negative."),   
        }
    }
    
    for board in &mut list_of_boards{
        board.populate_columns();
    }

    let mut board_points = 0;
    let mut winner_number = 0;
    'number_loop: for number in numbers_drawn{
        for board in &mut list_of_boards{
            if board.check_if_number_produces_bingo(number){
                println!("{number}");
                board.print_board();
                winner_number = number;
                board_points = board.calculate_sum_of_not_drawn_numbers();
                break 'number_loop;
            }
        }
    }    
    board_points * winner_number
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let header = &input[0];
    let mut numbers_drawn:Vec<i32> = Vec::new();
    for number in header.split(','){
        numbers_drawn.push(number.parse().expect("Couldn't parse!"));
    }

    let mut list_of_boards:Vec<Board> = Vec::new();

    let mut board_id = 0;
    for line in &input[1..]{
        if line.len() == 0 {
            list_of_boards.push(Board { columns: Vec::new(), rows: Vec::new(), id: board_id});
            board_id += 1;
        }
    }

    let mut current_board = 0;
    for line in &input[1..]{
        match line.len().cmp(&0) {
            Ordering::Equal => continue,
            Ordering::Greater => {
                let mut new_row = Row{elements: Vec::new()};
                for character in line.split(" "){
                    if character.is_empty(){continue;}
                    let new_number = Number{value: character.trim().parse().expect("Expected a number."), drawn: false};
                    new_row.add_element(new_number);
                }
                list_of_boards[current_board].rows.push(new_row);
                if list_of_boards[current_board].rows.len() == 5 {current_board +=1}
            },
            Ordering::Less => panic!("Lines cannot be negative."),   
        }
    }

    for board in &mut list_of_boards{
        board.populate_columns();
    }

    let number_of_boards = list_of_boards.len();

    let mut board_points = 0;
    let mut winner_number = 0;
    let mut list_of_winners:Vec<i32> = Vec::new();
    'number_loop: for number in numbers_drawn{
        for board in &mut list_of_boards{
            if list_of_winners.contains(&board.id){continue;} 
            if board.check_if_number_produces_bingo(number){
                if !list_of_winners.contains(&board.id){list_of_winners.push(board.id)}
                if list_of_winners.len() == number_of_boards{
                    println!("{number}");
                    board.print_board();
                    println!("");
                    board.print_state();
                    winner_number = number;
                    board_points = board.calculate_sum_of_not_drawn_numbers();
                    break 'number_loop;
                }
            }
        }
    }    
    board_points * winner_number

}

struct Board{
    rows:Vec<Row>,
    columns:Vec<Column>,
    id:i32,
}

impl Board{
    fn check_if_number_produces_bingo(&mut self, number: i32) -> bool{
        for row in &mut self.rows{
            for element in &mut row.elements{
                if number == element.value{element.was_drawn();}
            }
        }

        for column in &mut self.columns{
            for element in &mut column.elements{
                if number == element.value{element.was_drawn();}
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

    fn check_for_bingo(&mut self) -> bool{
        for row in &mut self.rows{
            if row.check_for_bingo(){return true;}
        }
        for column in &mut self.columns{
            
            if column.check_for_bingo(){return true;}
        }
        return false;
    }
    
    fn populate_columns(&mut self){
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
    fn check_for_bingo(&mut self) -> bool{
        let mut count = 0;
        for element in &self.elements{
            if element.is_drawn(){count += 1;}
        }
        if count == 5 {true} else {false}
    }
}

struct Column {
    elements:Vec<Number>,
}

impl Column {
    fn add_element(&mut self, number: Number){
        match self.elements.len() < 5 {
            true => self.elements.push(number),
            false => panic!("Board can only have 5 elements per column"),
        }
    }
    fn check_for_bingo(&mut self) -> bool{
        let mut count = 0;
        for element in &self.elements{
            if element.is_drawn(){count += 1;}
        }
        if count == 5 {true} else {false}
    }
}

#[derive(Clone)]
struct Number{
    value:i32,
    drawn:bool,
}

impl Number {
    fn was_drawn(&mut self){
        self.drawn = true;
    }
    fn is_drawn(&self) -> bool{
        self.drawn
    }
}