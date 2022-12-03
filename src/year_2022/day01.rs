pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 69528);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 1150);
}

fn solve_part1(input: &Vec<String>) -> i32 {

    let mut calories:i32 = 0;
    let mut elf_calories:Vec<i32> = Vec::new();
    for line in input.iter(){

        if line.len() == 0 {
            println!("Skipped");
            elf_calories.push(calories);
            calories = 0;
            continue;
        }
        calories += line.parse::<i32>().unwrap();
    }
    elf_calories.push(calories);
    *elf_calories.iter().max().unwrap()
}


fn solve_part2(input: &Vec<String>) -> i32 {

    let mut calories:i32 = 0;
    let mut elf_calories:Vec<i32> = Vec::new();
    for line in input.iter(){

        if line.len() == 0 {
            println!("Skipped");
            elf_calories.push(calories);
            calories = 0;
            continue;
        }
        calories += line.parse::<i32>().unwrap();
    }
    elf_calories.push(calories);
    elf_calories.sort();
    return elf_calories.iter().rev().take(3).sum::<i32>();
}
