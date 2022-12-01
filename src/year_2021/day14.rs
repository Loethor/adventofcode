use std::collections::HashMap;



pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 2851);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // // assert_eq!(part2, 10002813279337);
}
fn solve_part1(input: &Vec<String>) -> i64 {

    let polymer_template:String = input[0].to_string();
    let pair_generation_rules = obtain_pair_generation_rules(input);
    let mut ocurrences = initialize_ocurrences(&polymer_template);
    let mut number_of_pairs = obtain_number_of_pairs(polymer_template);
    const ITERATIONS: usize = 10;    
    for _ in 0..ITERATIONS{
        let mut new_number_of_pairs = number_of_pairs.clone();
        for key in  number_of_pairs.keys(){
            let pair_of_letters = &pair_generation_rules[key];
            *new_number_of_pairs.entry(pair_of_letters.0.clone()).or_insert(0) += number_of_pairs[key];
            *new_number_of_pairs.entry(pair_of_letters.1.clone()).or_insert(0) += number_of_pairs[key];
            *new_number_of_pairs.entry(key.clone()).or_insert(0) -= number_of_pairs[key];
            let letter = pair_of_letters.0.chars().nth(1).unwrap();
            let entry = ocurrences.entry(letter).or_insert(0);
            *entry += number_of_pairs[key];
        }
        number_of_pairs = new_number_of_pairs.clone();
    }
    ocurrences.values().max().unwrap() - ocurrences.values().min().unwrap()
}

fn solve_part2(input: &Vec<String>) -> i64 {

    let polymer_template:String = input[0].to_string();
    let pair_generation_rules = obtain_pair_generation_rules(input);
    let mut ocurrences = initialize_ocurrences(&polymer_template);
    let mut number_of_pairs = obtain_number_of_pairs(polymer_template);
    const ITERATIONS: usize = 40;
    for _ in 0..ITERATIONS{
        let mut new_number_of_pairs = number_of_pairs.clone();
        for key in  number_of_pairs.keys(){
            let pair_of_letters = &pair_generation_rules[key];
            *new_number_of_pairs.entry(pair_of_letters.0.clone()).or_insert(0) += number_of_pairs[key];
            *new_number_of_pairs.entry(pair_of_letters.1.clone()).or_insert(0) += number_of_pairs[key];
            *new_number_of_pairs.entry(key.clone()).or_insert(0) -= number_of_pairs[key];
            let letter = pair_of_letters.0.chars().nth(1).unwrap();
            let entry = ocurrences.entry(letter).or_insert(0);
            *entry += number_of_pairs[key];
        }
        number_of_pairs = new_number_of_pairs.clone();
    }
    ocurrences.values().max().unwrap() - ocurrences.values().min().unwrap()
}

fn obtain_number_of_pairs(polymer_template: String) -> HashMap<String, i64> {
    let mut number_of_pairs:HashMap<String,i64> = HashMap::new();
    for i in 0..polymer_template.len() - 1{
        let key = nth_char(&polymer_template, i) + &nth_char(&polymer_template, i+1);
        let entry = number_of_pairs.entry(key).or_insert(0);
        *entry += 1;
    }
    number_of_pairs
}

fn initialize_ocurrences(polymer_template: &String) -> HashMap<char, i64> {
    let mut ocurrences:HashMap<char,i64> = HashMap::new();
    for letter in polymer_template.chars(){
        let entry = ocurrences.entry(letter).or_insert(0);
        *entry += 1;
    }
    ocurrences
}

fn obtain_pair_generation_rules(input: &Vec<String>) -> HashMap<String, (String, String)> {
    let mut pair_generation_rules:HashMap<String, (String, String)> = HashMap::new();
    for line in &input[2..]{
        let (key,value) = line.split_once(" -> ").unwrap();
        let first_letter = nth_char(key, 0);
        let second_letter = nth_char(key, 1);
        let generated_letter = value.to_string();
        pair_generation_rules.insert(key.to_string(), (first_letter + &generated_letter, generated_letter + &second_letter));
    }
    pair_generation_rules
}

fn nth_char(string: &str, position:usize) -> String {
    string.chars().nth(position).unwrap().to_string()
}
