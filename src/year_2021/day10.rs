pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 389589);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // // assert_eq!(part2, 94813675);
}


fn solve_part1(input: &Vec<String>) -> u32 {
    let mut score = 0;


    
    for line in input{
        let mut open_chunk:Vec<char> = Vec::new();
        for character in line.chars(){
            // println!("{character} ");
            match character{
                '[' => open_chunk.push(character),
                '(' => open_chunk.push(character),
                '{' => open_chunk.push(character),
                '<' => open_chunk.push(character),
                ']' => {
                    if open_chunk.last().unwrap() == &'['{
                        open_chunk.pop();
                    } else {
                        score += 57;
                        break;
                    }
                },
                ')' => {
                    if open_chunk.last().unwrap() == &'('{
                        open_chunk.pop();
                    } else {
                        score += 3;
                        break;
                    }
                },
                '}' => {
                    if open_chunk.last().unwrap() == &'{'{
                        open_chunk.pop();
                    } else {
                        score += 1197;
                        break;
                    }
                },
                '>' => {
                    if open_chunk.last().unwrap() == &'<'{
                        open_chunk.pop();
                    } else {
                        score += 25137;
                        break;
                    }
                },
                _ => panic!("Wrong input.")
            }
        }
    }

    score
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut scores:Vec<u64> = Vec::new();

    let mut uncorrupted_input = input.clone();

    
    'a:for line in input{
        let mut open_chunk:Vec<char> = Vec::new();
        for character in line.chars(){
            match character{
                '[' => open_chunk.push(character),
                '(' => open_chunk.push(character),
                '{' => open_chunk.push(character),
                '<' => open_chunk.push(character),
                ']' => {
                    if open_chunk.last().unwrap() == &'['{
                        open_chunk.pop();
                    } else {
                        continue 'a;
                    }
                },
                ')' => {
                    if open_chunk.last().unwrap() == &'('{
                        open_chunk.pop();
                    } else {
                        continue 'a;
                    }
                },
                '}' => {
                    if open_chunk.last().unwrap() == &'{'{
                        open_chunk.pop();
                    } else {
                        continue 'a;
                    }
                },
                '>' => {
                    if open_chunk.last().unwrap() == &'<'{
                        open_chunk.pop();
                    } else {
                        continue 'a;
                    }
                },
                _ => panic!("Wrong input.")
            }
        }
        let mut score = 0;
        for elem in open_chunk.iter().rev(){
            match elem {
                '(' => score = score * 5 + 1,
                '[' => score = score * 5 + 2,
                '{' => score = score * 5 + 3,
                '<' => score = score * 5 + 4,
                _ => panic!("Wrong chunk.")
            }
        }
        scores.push(score);
    }

    println!("{}", scores.len()/2);

    scores.sort();
    for score in &scores{
        println!("{score}")
    }
    

    return scores[scores.len()/2]
}