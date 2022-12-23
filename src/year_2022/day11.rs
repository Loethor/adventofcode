// This file is just a template for any day solution.
use std::{time::{Instant}, collections::{VecDeque, HashMap}};

pub fn run(input: Vec<String>) {
    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1647);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

    let start = Instant::now();
    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2447);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

}

fn solve_part1(input: &Vec<String>) -> i64 {

    let mut monkey_map:HashMap<usize, (Monkey, usize, usize)> = HashMap::new();
    let _ = populate_monkey_vector(input, &mut monkey_map);
    let number_of_monkeys = monkey_map.len();
    let rounds = 20;

    for _ in 0..rounds {
        for i in 0..number_of_monkeys{
            execute_monkey_turn_part1(&mut monkey_map, i);
        }
    }
    let mut inspections = obtain_inspections(number_of_monkeys, monkey_map);
    inspections.sort();
    return inspections.iter().rev().take(2).product::<i64>();
}

fn execute_monkey_turn_part1(monkey_map: &mut HashMap<usize, (Monkey, usize, usize)>, i: usize) {
    let mut cp0 = monkey_map.clone();
    let mut cp1 = monkey_map.clone();
    let mut cp2 = monkey_map.clone();
    let (my_monkey, t1, t2) = cp0.get_mut(&i).unwrap();
    let (monkey_1, a ,b ) = cp1.get_mut(&t1).unwrap();
    let (monkey_2, c ,d ) = cp2.get_mut(&t2).unwrap();
    monkey_business_part1(my_monkey,  monkey_1,  monkey_2);
    *monkey_map.entry(i).or_insert((my_monkey.clone(), *t1, *t2 )) = (my_monkey.clone(), *t1, *t2 );
    *monkey_map.entry(*t1).or_insert((monkey_1.clone(), *a, *b )) = (monkey_1.clone(), *a, *b );
    *monkey_map.entry(*t2).or_insert((monkey_2.clone(), *c, *d )) = (monkey_2.clone(), *c, *d );
}

fn solve_part2(input: &Vec<String>) -> i64 {

    let mut monkey_map:HashMap<usize, (Monkey, usize, usize)> = HashMap::new();
    let lcd = populate_monkey_vector(input, &mut monkey_map);
    let number_of_monkeys = monkey_map.len();
    let rounds = 10_000;

    for _ in 0..rounds {
        for i in 0..number_of_monkeys{
            execute_monkey_turn_part2(&mut monkey_map, i, lcd);
        }
    }
    let mut inspections = obtain_inspections(number_of_monkeys, monkey_map);
    inspections.sort();
    return inspections.iter().rev().take(2).product::<i64>();
}

fn obtain_inspections(number_of_monkeys: usize, monkey_map: HashMap<usize, (Monkey, usize, usize)>) -> Vec<i64> {
    let mut inspections = Vec::new();
    for i in 0..number_of_monkeys{
        let a = monkey_map.get(&i).unwrap().0.inspections;
        inspections.push(a);
    }
    inspections
}


fn execute_monkey_turn_part2(monkey_map: &mut HashMap<usize, (Monkey, usize, usize)>, i: usize, lcd: i64) {
    let mut cp0 = monkey_map.clone();
    let mut cp1 = monkey_map.clone();
    let mut cp2 = monkey_map.clone();
    let (my_monkey, t1, t2) = cp0.get_mut(&i).unwrap();
    let (monkey_1, a ,b ) = cp1.get_mut(&t1).unwrap();
    let (monkey_2, c ,d ) = cp2.get_mut(&t2).unwrap();
    monkey_business_part2(my_monkey,  monkey_1,  monkey_2, lcd);
    *monkey_map.entry(i).or_insert((my_monkey.clone(), *t1, *t2 )) = (my_monkey.clone(), *t1, *t2 );
    *monkey_map.entry(*t1).or_insert((monkey_1.clone(), *a, *b )) = (monkey_1.clone(), *a, *b );
    *monkey_map.entry(*t2).or_insert((monkey_2.clone(), *c, *d )) = (monkey_2.clone(), *c, *d );
}

fn monkey_business_part1(m1:&mut Monkey, m2:&mut Monkey, m3:&mut Monkey) {
    let size = m1.items.len();
    if size == 0 {return;}
    for _ in 0..size{
        m1.inspect_item_division();
        if m1.test_item(){
            m1.throw_item(m2);
        } else {
            m1.throw_item(m3);
        }
    }
}

fn monkey_business_part2(m1:&mut Monkey, m2:&mut Monkey, m3:&mut Monkey, lcd:i64) {
    let size = m1.items.len();
    if size == 0 {return;}
    for _ in 0..size{
        m1.inspect_item_modulus(lcd);
        if m1.test_item(){
            m1.throw_item(m2);
        } else {
            m1.throw_item(m3);
        }
    }
}

fn populate_monkey_vector(input: &Vec<String>,
                    monkey_map: &mut HashMap<usize, (Monkey, usize, usize)>,) -> i64{

    let mut monkey_id:usize = 0;
    let mut list_of_items:VecDeque<i64> = VecDeque::new();
    let mut inspect_worry_factor:i64 = 0;
    let mut monkey_operation:Operation = Operation::Add;
    let mut test_factor:i64 = 0;
    let mut targets:[usize; 2] = [0,0];
    let mut lcd:i64 = 1;
    for line in input.iter(){
    
        if line.contains("Monkey") {
            continue;
        } else if line.contains("Starting"){
            generate_starting_items(line, &mut list_of_items);
        } else if line.contains("Operation") {
            generate_operation(line, &mut inspect_worry_factor, &mut monkey_operation);   
        } else if line.contains("Test") {
            generate_test_factor(line, &mut test_factor);
            lcd *= test_factor;
        } else if line.contains("true") {
            generate_true_target(line, &mut targets);
        } else if line.contains("false") {
            generate_false_target(line, &mut targets);
        } else {
            let new_monkey = generate_monkey(monkey_id, &list_of_items, &monkey_operation, inspect_worry_factor, test_factor);
            monkey_map.insert(monkey_id, (new_monkey, targets[0], targets[1]));
            list_of_items = VecDeque::new();
            monkey_id += 1;
        }
    }
    let new_monkey = generate_monkey(monkey_id, &list_of_items, &monkey_operation, inspect_worry_factor, test_factor);
    monkey_map.insert(monkey_id, (new_monkey, targets[0], targets[1]));
    lcd
}

fn generate_monkey(monkey_id: usize, list_of_items: &VecDeque<i64>, monkey_operation: &Operation, inspect_worry_factor: i64, test_factor: i64) -> Monkey {
    let new_monkey = Monkey {
        id:monkey_id,
        items:list_of_items.clone(),
        operation:monkey_operation.clone(),
        inspect_worry_factor,
        test_factor,
        inspections:0,
    };
    new_monkey
}

fn generate_false_target(line: &String, targets: &mut [usize; 2]) {
    let (_, num) = line.split_once("monkey ").unwrap();
    targets[1] = num.parse().unwrap();
}

fn generate_true_target(line: &String, targets: &mut [usize; 2]) {
    let (_, num) = line.split_once("monkey ").unwrap();
    targets[0] = num.parse().unwrap();
}

fn generate_test_factor(line: &String, test_factor: &mut i64) {
    let (_, num) = line.split_once("by ").unwrap();
    *test_factor = num.parse().unwrap();
}

fn generate_operation(line: &String, inspect_worry_factor: &mut i64, monkey_operation: &mut Operation) {
    let (_, slice) = line.split_once("= ").unwrap();
    if slice.contains("+"){
        let (_, num) = slice.split_once(" + ").unwrap();
        *inspect_worry_factor = num.parse().unwrap();
        *monkey_operation = Operation::Add;
    } else {
        let (_, num) = slice.split_once(" * ").unwrap();
        if num == "old" {
            *monkey_operation = Operation::MultItself;
        } else {
            *inspect_worry_factor = num.parse().unwrap();
            *monkey_operation = Operation::Mult;
        }
    }
}

fn generate_starting_items(line: &String, list_of_items: &mut VecDeque<i64>) {
    let (_, slice) = line.split_once(": ").unwrap();
    if slice.contains(","){
        for item in slice.split(", "){
            list_of_items.push_back(item.parse().unwrap());
        }
    } else {
        list_of_items.push_back(slice.parse().unwrap());
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Mult,
    MultItself,
} 

impl Operation {
    fn operate(&self, old:i64, operand:i64) -> i64 {
        match self {
            Operation::Add => old + operand,
            Operation::Mult => old * operand,
            Operation::MultItself => old * old,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    id:usize,
    items:VecDeque<i64>,
    operation:Operation,
    inspect_worry_factor:i64,
    test_factor:i64,
    inspections:i64,
} 

impl Monkey {
    fn operate(&mut self, monkey_1: &mut Monkey, monkey_2: &mut Monkey )  {

    }

    fn receive_item(&mut self, item:i64) {
        self.items.push_back(item);
    }

    fn throw_item(&mut self, other: &mut Monkey){
        let item = self.items.pop_front().unwrap();
        other.receive_item(item)
    }

    fn inspect_item_division(&mut self) {
        self.inspections += 1;
        let mut my_item = self.items[0];
        my_item = self.operation.operate(my_item, self.inspect_worry_factor);
        my_item = my_item / 3;
        self.items[0] = my_item;
    }

    fn inspect_item_modulus(&mut self, lowest_common_denominator:i64) {
        self.inspections += 1;
        let mut my_item = self.items[0];
        my_item = self.operation.operate(my_item, self.inspect_worry_factor);
        my_item = my_item % lowest_common_denominator;
        self.items[0] = my_item;
    }

    fn test_item(&self)-> bool {
        if self.items[0] % self.test_factor == 0 {
            true
        } else {
            false
        }
    }

    
}