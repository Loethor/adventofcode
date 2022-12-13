pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 464);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 770);
}

fn solve_part1(input: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in input.iter(){
        let (first_half, second_half) = line.split_once(',').unwrap();
        
        let elf_one = obtain_assigment_list(first_half);
        let elf_two = obtain_assigment_list(second_half);
        if elf_one.len() > elf_two.len() {
            if big_fully_contains_small(&elf_two, &elf_one){
                count += 1;
            }
        } else {
            if big_fully_contains_small(&elf_one, &elf_two){
                count += 1;
            }
        }
    }
    count
}

fn big_fully_contains_small(small_assignment: &Vec<i32>, big_assingment: &Vec<i32>, ) -> bool {
    for assignment in small_assignment {
        if !big_assingment.contains(&assignment) {
            return false;
        }
    }
    return true;
}

fn obtain_assigment_list(assigment_str: &str) -> Vec<i32>{
    let mut elf_assigment_list = Vec::new();
    let (begin, end) = assigment_str.split_once('-').unwrap();
    let (begin, end):(i32,i32) = (begin.parse().unwrap(), end.parse().unwrap());
    for i in begin..=end{
        elf_assigment_list.push(i);
    }
    elf_assigment_list    
}

fn solve_part2(input: &Vec<String>) -> u32 {
    let mut count = 0;
    for line in input.iter(){
        let (first_half, second_half) = line.split_once(',').unwrap();
        
        let elf_one = obtain_assigment_list(first_half);
        let elf_two = obtain_assigment_list(second_half);
        if any_overlap(&elf_two, &elf_one){
            count += 1;
        }
    }
    count
}

fn any_overlap(small_assignment: &Vec<i32>, big_assingment: &Vec<i32>, ) -> bool {
    for element in big_assingment{
        if small_assignment.contains(element){
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assigment_list() {
        let test_asigment = "2-4";
        let test_elf = obtain_assigment_list(test_asigment);
        assert_eq!(test_elf.len(), 3);
        assert_eq!(test_elf[0], 2);
        assert_eq!(test_elf[1], 3);
        assert_eq!(test_elf[2], 4);
    }

    #[test]
    fn test_big_fully_contains_small(){
        let test_big_asigment = vec![2,3,4,5,6];
        let test_small_asigment = vec![3,4];

        assert_eq!(big_fully_contains_small(&test_small_asigment, &test_big_asigment), true);
        assert_eq!(big_fully_contains_small(&test_big_asigment, &test_small_asigment), false);
    }
    
    #[test]
    fn test_any_overlap(){
        let test_big_asigment = vec![2,3,4,5,6];
        let test_small_asigment = vec![3,4];
        let test_not_overlaping = vec![1];

        assert_eq!(any_overlap(&test_small_asigment, &test_big_asigment), true);
        assert_eq!(any_overlap(&test_big_asigment, &test_small_asigment), true);
        assert_eq!(any_overlap(&test_not_overlaping, &test_small_asigment), false);
        assert_eq!(any_overlap(&test_not_overlaping, &test_big_asigment), false);
    }
}