use std::{collections::HashSet, cmp};

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1647);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2447);
}

fn solve_part1(input: &Vec<String>) -> usize {
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(tail);

    for line in input.iter(){
        let (direction, steps) = line.split_once(" ").unwrap();
        let direction = Direction::str_to_direction(direction);
        let steps:i32 = steps.parse().unwrap();
        process_move(direction, steps, &mut head, &mut tail, &mut visited);
    }

    // // for fun
    // print_movement(&visited);
    visited.len()
}

#[allow(dead_code)]
fn print_movement(visited: &HashSet<Position>) {
    let (max_i, max_j) = calculate_max_dim(visited);
    let (min_i, min_j) = calculate_min_dim(visited);
    for j in (min_j..max_j).rev(){
        for i in min_i..max_i{
            let pos = Position{x:i, y:j};
            if (i,j) == (0,0) {
                print!("s ");
            }
            else if visited.contains(&pos){
                print!("# ");
            } else {
                print!(". ")
            }
        }
        println!("");
    }
}



#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn str_to_direction(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("str_to_direction(): Invalid &str, no Direction can be produced.")
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }
}

fn move_head_towards_direction(dir: Direction, pos: &mut Position, ) {
    match dir {
        Direction::Up => { pos.y += 1; }
        Direction::Right => { pos.x += 1; }
        Direction::Down => { pos.y -= 1; }
        Direction::Left => { pos.x -= 1; }
    }
}

fn obtain_max_delta_pos(origin: Position, target: Position) -> u32 {
    let delta_x: u32 = (target.x - origin.x).abs() as u32;
    let delta_y: u32 = (target.y - origin.y).abs() as u32;
    return cmp::max(delta_x, delta_y);
}

fn obtain_tail_movement(origin: Position, target: Position) -> Position {
    let mut delta_x: i32 = target.x - origin.x;
    let mut delta_y: i32 = target.y - origin.y;
    if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
        delta_x = delta_x.clamp(-1, 1);
        delta_y = delta_y.clamp(-1, 1);
    } else if delta_x.abs() == 2 && delta_y == 0 {
        delta_x = delta_x.clamp(-1, 1);
    } else if delta_x == 0 && delta_y.abs() == 2 {
        delta_y = delta_y.clamp(-1, 1);
    }
    Position{ x: delta_x, y: delta_y }
}

fn process_move(direction: Direction, steps: i32, head: &mut Position, tail: &mut Position, visited: &mut HashSet<Position>) {
    for _ in 1..=steps {
        move_head_towards_direction(direction, head);
        if obtain_max_delta_pos(*tail, *head) > 1 {
            let delta = obtain_tail_movement(*tail, *head);
            tail.x += delta.x;
            tail.y += delta.y;
            visited.insert(*tail);
        }
    }
}

fn calculate_max_dim(set:&HashSet<Position>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for pos in set{
        if pos.x > max_x{
            max_x = pos.x;
        }
        if pos.y > max_y{
            max_y = pos.y;
        }
    }
    (max_x + 1, max_y + 1)
}

fn calculate_min_dim(set:&HashSet<Position>) -> (i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    for pos in set{
        if pos.x < min_x{
            min_x = pos.x;
        }
        if pos.y < min_y{
            min_y = pos.y;
        }
    }
    (min_x, min_y)
}

fn solve_part2(input: &Vec<String>) -> usize {
    let mut rope: Vec<Position> = vec![Position::new(); 10];
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(tail);

    for line in input.iter(){
        let (direction, steps) = line.split_once(" ").unwrap();
        let direction = Direction::str_to_direction(direction);
        let steps:i32 = steps.parse().unwrap();
        process_long_move(direction, steps, &mut rope, &mut visited);
    }

    // // for fun
    // print_movement(&visited);
    visited.len()
}

fn process_long_move(direction: Direction, steps: i32, rope: &mut Vec<Position>, visited: &mut HashSet<Position>) {
    let rope_len = rope.len();
    for _ in 1..=steps {
        move_head_towards_direction(direction, &mut rope[0]);
        for i in 1..rope_len{
            if obtain_max_delta_pos(rope[i], rope[i-1]) > 1 {
                let delta = obtain_tail_movement(rope[i], rope[i-1]);
                rope[i].x += delta.x;
                rope[i].y += delta.y;
                if i == rope_len - 1{
                    visited.insert(rope[i]);
                }
            }
        }
        
    }
}