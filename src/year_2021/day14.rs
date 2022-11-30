use std::collections::HashMap;



pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1702);


    // let part2 = solve_part2(&input);
    // println!("part 2: {}", part2);
    // // // assert_eq!(part2, 94813675);
}
fn solve_part1(input: &Vec<String>) -> usize {

    let mut polymer_template:String = input[0].clone();
    println!("{polymer_template}");

    let mut pair_insertion_rules:HashMap<String, String> = HashMap::new();


    for line in &input[2..]{

        let (k,v) = line.split_once(" -> ").unwrap();
        pair_insertion_rules.insert(k.to_string(), v.to_string());
        // println!("{line}")
    }

    println!("{:?}",pair_insertion_rules);

    for _ in 0..1{
        let mut new_polymer = String::new();
        let inter = polymer_template.chars().collect::<Vec<char>>();
        let mut windows = inter.windows(2);
    
        for pair in windows{
            print!("{:?}", pair);
        }
    }

    0
}
