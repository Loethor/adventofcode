

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 7553);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 2758);
}

fn solve_part1(input: &Vec<String>) -> u32 {


    let mut points = 0;
    let mut list_of_repeated_letters:Vec<char> = Vec::new();
    for line in input.iter(){

        let (first_half,second_half) = split_string_in_half(line);

        match find_intersecting_two_strings(first_half, second_half) {
            Some(letter) => list_of_repeated_letters.push(letter),
            None => panic!("The arrays have no intersection"),
        }    

    }

    for letter in list_of_repeated_letters{
        points += letter_to_points(letter);
    }

    points
}

fn solve_part2(input: &Vec<String>) -> u32 {

    let mut points = 0;
    let mut list_of_repeated_letters:Vec<char> = Vec::new();
    let mut counter = 0;

    let mut first_elf = String::new();
    let mut second_elf = String::new();
    for line in input.iter(){

        match counter {
            0 => {first_elf = line.clone(); counter += 1;}
            1 => {second_elf = line.clone(); counter += 1;}
            2 => {
                    let third_elf = line.clone();
                    counter = 0;
                    let letter = find_intersecting_three_strings(&first_elf, &second_elf, &third_elf);
                    list_of_repeated_letters.push(
                        match letter {
                            Some(l) => l,
                            None => panic!("Intersection couldn't be found.")
                        }
                    )
                }
            _ => panic!("Something went wrong with the counter.")
        }
    }

    for letter in list_of_repeated_letters{
        points += letter_to_points(letter);
    }

    points
}


fn find_intersecting_two_strings(first_half: &str, second_half: &str) ->Option<char>{
    for letter in first_half.chars(){
        if second_half.contains(letter){
            return Some(letter);
        }
    }
    None
}

fn find_intersecting_three_strings(first: &str, second: &str, third: &str) ->Option<char>{
    for letter in first.chars(){
        if second.contains(letter) && third.contains(letter){
            return Some(letter);
        }
    }
    None
}


fn letter_to_points(letter:char) -> u32 {
    if letter.is_ascii_uppercase(){
        return letter as u32 - 38;
    } else {
        return letter as u32 - 96;
    }
}


fn split_string_in_half(string: &String) -> (&str, &str){
    let half_lenght = string.len() / 2;
    let (f,s) = string.split_at(half_lenght);
    (f,s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_string_in_half() {
        let t_string = String::from("test");
        let (f_half,s_half) = split_string_in_half(&t_string);
        assert_eq!(f_half, "te");
        assert_eq!(s_half, "st");
    }

    #[test]
    fn test_find_intersection_two_succeds(){
        let t_string = String::from("test");
        let t_string2 = String::from("toad");

        let e_char = find_intersecting_two_strings(&t_string, &t_string2);

        assert_eq!(e_char, Some('t'));
    }

    #[test]
    fn test_find_intersection_two_fails(){
        let t_string = String::from("test");
        let t_string2 = String::from("dog");

        let e_char = find_intersecting_two_strings(&t_string, &t_string2);

        assert_eq!(e_char, None);
    }

    #[test]
    fn test_find_intersection_three_succeds(){
        let t_string = String::from("test");
        let t_string2 = String::from("toad");
        let t_string3 = String::from("tasty");

        let e_char = find_intersecting_three_strings(&t_string, &t_string2, &t_string3);

        assert_eq!(e_char, Some('t'));
    }

    #[test]
    fn test_letter_to_points(){
        let lowercase_char = 'a';
        let upper_case = 'A';

        let e_lowercase = letter_to_points(lowercase_char);
        let e_uppercase = letter_to_points(upper_case);

        assert_eq!(e_uppercase, 65-38);
        assert_eq!(e_lowercase, 97-96);
    }
}