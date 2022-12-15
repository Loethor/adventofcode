use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1647);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 2447);
}

fn solve_part1(input: &Vec<String>) -> usize {
    const NUMBER_OF_NON_REPEATING:usize = 4;
    let mut counter = NUMBER_OF_NON_REPEATING - 1;
    let line = &input[0];
    let chars_vec:Vec<char> = line.chars().collect();
    for window_of_characters in chars_vec.windows(NUMBER_OF_NON_REPEATING){
        counter += 1;
        if has_unique_chars(window_of_characters) {
            break;
        }
    }
    counter
}

fn solve_part2(input: &Vec<String>) -> usize {
    const NUMBER_OF_NON_REPEATING:usize = 14;
    let mut counter = NUMBER_OF_NON_REPEATING - 1;
    let line = &input[0];
    let chars_vec:Vec<char> = line.chars().collect();
    for window_of_characters in chars_vec.windows(NUMBER_OF_NON_REPEATING){
        counter += 1;
        if has_unique_chars(window_of_characters) {
            break;
        }
    }
    counter
}

fn has_unique_chars(char_vec:&[char]) -> bool {
    let mut char_set:HashSet<char> = HashSet::new();
    for ch in char_vec{
        if !char_set.contains(ch){
            char_set.insert(*ch);
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_unique_chars_good_weather() {
        // Arrange
        let test_unique_chars = vec!['a','b','c'];

        // Act
        let test_bool = has_unique_chars(&test_unique_chars);

        // Assert
        assert_eq!(test_bool, true);
    }

    #[test]
    fn test_has_unique_chars_bad_weather() {
        // Arrange
        let test_repeating_chars = vec!['a','b','b'];
        
        // Act
        let test_bool = has_unique_chars(&test_repeating_chars);

        // Assert
        assert_eq!(test_bool, false);
    }
}