pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1702);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // // assert_eq!(part2, 94813675);
}

fn solve_part1(input: &Vec<String>) -> u32 {
    let m = input.len();
    let n = input[0].len();
    let mut squares :Vec<Square> = Vec::new();
    for line in input{
        for number in line.chars(){
            squares.push( Square { value: number.to_digit(10).unwrap()})
        }
    }

    let mut flashes = 0;
    for d in 0..100{

        println!("Day {} before +1",d);
        for i in 0..m{
            for j in 0..n{
                print!("{} ",squares[i*n+j].value);
            }
            print!("\n");
        }

        for i in 0..m{
            for j in 0..n{
                squares[i*n+j].value += 1;
            }
        }

        println!("Day {} after +1",d);
        for i in 0..m{
            for j in 0..n{
                print!("{} ",squares[i*n+j].value);
            }
            print!("\n");
        }


        let mut nines = 1;
        'a:while nines > 0 {
            for i in 0..m{
                for j in 0..n{
                    if squares[i*n+j].value > 9 {
                        flashes += 1;
                        if i > 0{
                            if j > 0{
                                if squares[(i-1)*n+(j-1)].value != 0{
                                    squares[(i-1)*n+(j-1)].value += 1;
                                }
                            }
                            if j < n-1{
                                if squares[(i-1)*n+(j+1)].value != 0{
                                    squares[(i-1)*n+(j+1)].value += 1;
                                }
                            }
                            if squares[(i-1)*n+(j)].value != 0{
                                squares[(i-1)*n+(j)].value += 1;
                            }

                        }
                        if i < m - 1 {
                            if j > 0{
                                if squares[(i+1)*n+(j-1)].value != 0{
                                    squares[(i+1)*n+(j-1)].value += 1;
                                }
                            }
                            if j < n-1{
                                if squares[(i+1)*n+(j+1)].value != 0{
                                    squares[(i+1)*n+(j+1)].value += 1;
                                }
                            }
                            if squares[(i+1)*n+(j)].value != 0{
                                squares[(i+1)*n+(j)].value += 1;
                            }
                        }

                        if j > 0{
                            if squares[(i)*n+(j-1)].value != 0{
                                squares[(i)*n+(j-1)].value += 1;
                            }
                        }
                        if j < n-1{
                            if squares[(i)*n+(j+1)].value != 0{
                                squares[(i)*n+(j+1)].value += 1;
                            }
                        }
                        squares[i*n+j].value = 0;
                        continue 'a;
                    }
    
                }
            }
            nines = 0;
        }
        println!("Day {}",d);
        for i in 0..m{
            for j in 0..n{
                print!("{} ",squares[i*n+j].value);
            }
            print!("\n");
        }


    }

    flashes
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let m = input.len();
    let n = input[0].len();
    let mut squares :Vec<Square> = Vec::new();
    for line in input{
        for number in line.chars(){
            squares.push( Square { value: number.to_digit(10).unwrap()})
        }
    }

    let mut all_zero = false;
    let mut d = -1;
    while !all_zero{
        d +=1;

        println!("Day {} before +1",d);
        for i in 0..m{
            for j in 0..n{
                print!("{} ",squares[i*n+j].value);
            }
            print!("\n");
        }

        for i in 0..m{
            for j in 0..n{
                squares[i*n+j].value += 1;
            }
        }

        println!("Day {} after +1",d);
        for i in 0..m{
            for j in 0..n{
                print!("{} ",squares[i*n+j].value);
            }
            print!("\n");
        }


        let mut nines = 1;
        'a:while nines > 0 {
            for i in 0..m{
                for j in 0..n{
                    if squares[i*n+j].value > 9 {
                        if i > 0{
                            if j > 0{
                                if squares[(i-1)*n+(j-1)].value != 0{
                                    squares[(i-1)*n+(j-1)].value += 1;
                                }
                            }
                            if j < n-1{
                                if squares[(i-1)*n+(j+1)].value != 0{
                                    squares[(i-1)*n+(j+1)].value += 1;
                                }
                            }
                            if squares[(i-1)*n+(j)].value != 0{
                                squares[(i-1)*n+(j)].value += 1;
                            }

                        }
                        if i < m - 1 {
                            if j > 0{
                                if squares[(i+1)*n+(j-1)].value != 0{
                                    squares[(i+1)*n+(j-1)].value += 1;
                                }
                            }
                            if j < n-1{
                                if squares[(i+1)*n+(j+1)].value != 0{
                                    squares[(i+1)*n+(j+1)].value += 1;
                                }
                            }
                            if squares[(i+1)*n+(j)].value != 0{
                                squares[(i+1)*n+(j)].value += 1;
                            }
                        }

                        if j > 0{
                            if squares[(i)*n+(j-1)].value != 0{
                                squares[(i)*n+(j-1)].value += 1;
                            }
                        }
                        if j < n-1{
                            if squares[(i)*n+(j+1)].value != 0{
                                squares[(i)*n+(j+1)].value += 1;
                            }
                        }
                        squares[i*n+j].value = 0;
                        continue 'a;
                    }
    
                }
            }
            nines = 0;
        }

        all_zero = true;
        println!("Day {}",d);
        for i in 0..m{
            for j in 0..n{
                if squares[i*n+j].value != 0{
                    all_zero = false
                };
            }
            print!("\n");
        }


    }

    println!("We stopped at {}",d+1);
    d+1
}

#[derive(Debug)]
struct Square {
    value:u32,
}