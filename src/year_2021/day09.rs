use std::borrow::Borrow;

pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 548);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 94813675);
}


fn solve_part1(input: &Vec<String>) -> u32 {
    let mut count = 0;
    let m = input.len();
    let n = input[0].len();
    let mut squares :Vec<Square> = Vec::new();

    for line in input{
        for number in line.chars(){
            squares.push( Square { value: number.to_digit(10).unwrap(), state: State::Rest })
        }
    }

    let mut board:Board = Board { m, n, squares , basins: Vec::new() };
    board.find_ceiling();
    board.find_floor();

    for i in 0..m{
        for j in 0..n{
            let square = &board.squares[i*n+j];
            if square.state == State::Floor{count += 1 + square.value;}
        }
    }
    count
}


fn solve_part2(input: &Vec<String>) -> i64 {
    
    let mut count = 0;
    let m = input.len();
    let n = input[0].len();
    let mut squares :Vec<Square> = Vec::new();
    for line in input{
        for number in line.chars(){
            squares.push( Square { value: number.to_digit(10).unwrap(), state: State::Rest })
        }
    }

    let mut board:Board = Board { m, n, squares , basins: Vec::new() };

    board.find_ceiling();
    board.find_floor();
    board.find_basins();

    board.basins.sort();

    count = board.basins.iter().rev().take(3).product();
    count
}


struct Board {
    m:usize,
    n:usize,
    squares:Vec<Square>,
    basins:Vec<i64>,

} 

impl Board{
    fn find_floor(&mut self){
        
        let n = self.n;
        let m = self.m;
        
        for i in 0..m{
            for j in 0..n{

                if self.squares[i*n+j].state == State::Ceiling{continue;}
                let mut up:bool = false;
                let mut left:bool = false;
                let mut right:bool = false;
                let mut down:bool = false;

                if i > 0 {
                    if self.squares[(i-1) * n + j].value > self.squares[(i) * n + j].value {
                        up = true;
                    }
                }
                else {
                    up = true
                }
            
                if i < m - 1 {
                    if self.squares[(i+1) * n + j].value > self.squares[(i) * n + j].value {
                        down = true;
                    }
                } else{
                    down = true
                }
            
                if j > 0 {
                    if self.squares[(i) * n + j - 1].value > self.squares[(i) * n + j].value {
                        left = true;
                    }
                } else {
                    left = true
                }
            
                if j < n - 1 {
                    if self.squares[(i) * n + j+1].value > self.squares[(i) * n + j].value {
                        right = true;
                    }
                } else{
                    right = true
                }
                if up && left && right && down {self.squares[(i)*n+j].state = State::Floor}
            }
        }
        
        
    
    

    }
  
    fn find_ceiling(&mut self){
        let n = self.n;
        let m = self.m;
        for i in 0..m{
            for j in 0..n{
                if self.squares[(i) * n + j].value == 9 {
                    self.squares[(i) * n + j].state = State::Ceiling;
                }
            }
        }
    }

    fn find_basins(&mut self){
        println!("Inside Basins");
        let mut basins_local:Vec<i64> = Vec::new();

        let n = self.n;
        let m = self.m;
        for i in 0..m{
            for j in 0..n{
                if self.squares[(i) * n + j].state == State::Floor{
                    let number = self.create_basin(i,j);
                    basins_local.push(number);                    
                }
            }
        }
        self.basins = basins_local;
    }

    fn create_basin(&mut self, i:usize,j:usize) -> i64{

        let mut count = 1;
        let n = self.n;
        let m = self.m;
        if i > 0 {
            if self.squares[(i-1) * n + j].state == State::Rest {
                self.squares[(i-1) * n + j].state = State::Basin;
                count += self.create_basin(i-1, j);
            }
        }
    
        if i < m - 1 {
            if self.squares[(i + 1) * n + j].state == State::Rest {
                self.squares[(i + 1) * n + j].state = State::Basin;
                count += self.create_basin(i+1, j);
            }
        }
    
        if j > 0 {
            if self.squares[(i) * n + j - 1].state == State::Rest  {
                self.squares[(i) * n + j - 1].state = State::Basin;
                count += self.create_basin(i, j-1);
            }
        }
    
        if j < n - 1 {
            if self.squares[(i) * n + j + 1].state == State::Rest {
                self.squares[(i) * n + j + 1].state = State::Basin;
                count += self.create_basin(i, j+1);
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print_board_states(&self){
        let n = self.n;
        let m = self.m;
        for i in 0..m{
            for j in 0..n{
                if self.squares[i*n+j].state == State::Floor{
                print!("i:{}, j:{}, state:{:?} ",i,j,self.squares[i*n+j].state);
                }
            }
            println!("\n");
        }
    }

    
}

struct Square {
    value:u32,
    state:State,
}

#[derive(Debug,PartialEq)]
enum State {
    Floor,
    Basin,
    Rest,
    Ceiling,
}