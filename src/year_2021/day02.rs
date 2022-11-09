pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1660158);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1604592846);
}

fn solve_part1(input: &Vec<String>) -> i64 {
    let mut x_position: i64 = 0;
    let mut y_position: i64 = 0;
    for line in input.iter(){
        let mut world_iterable =  line.split_whitespace();
        let world = world_iterable.next().unwrap();
        let number = world_iterable.next().unwrap().parse::<i64>().unwrap();
        match world{
            "forward" => x_position += number,
            "up" => y_position -= number,
            "down" => y_position += number,
            _ => panic!("Word not recognized")
        } 
    }
    return x_position * y_position;
}

fn solve_part2(input: &Vec<String>) -> i64 {
    let mut aim: i64 = 0;
    let mut x_position: i64 = 0;
    let mut y_position: i64 = 0;
    for line in input.iter(){
        let mut world_iterable =  line.split_whitespace();
        let world = world_iterable.next().unwrap();
        let number = world_iterable.next().unwrap().parse::<i64>().unwrap();
        match world{
            "forward" =>
            {
                x_position += number; 
                y_position += number*aim
            },
            "up" => aim -= number,
            "down" => aim += number,
            _ => panic!("Word not recognized")
        } 
    }
    return x_position * y_position;
}