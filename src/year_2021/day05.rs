pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 6841);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 19258);
}


fn solve_part1(input: &Vec<String>) -> i32 {
    let mut width = 0;
    let mut height = 0;

    let mut list_of_numbers: Vec<Numbers> = Vec::new();
    for line in input{
        let mut number_to_add:Vec<i32> = Vec::new();
        for half in line.split(" -> "){
            for number in half.split(","){
                number_to_add.push(number.parse().expect("Expected a number."))
            }
        }
        increase_maximum(number_to_add[0], &mut width);
        increase_maximum(number_to_add[1], &mut height);
        increase_maximum(number_to_add[2], &mut width);
        increase_maximum(number_to_add[3], &mut height);


        list_of_numbers.push(Numbers{
            x1: number_to_add[0], 
            y1: number_to_add[1],
            x2: number_to_add[2],
            y2: number_to_add[3],
        });
    }
    let cap = ((height+1)*(width+1)) as usize;
    let mut matrix:Matrix = Matrix::new(height as usize +1, width as usize +1, Vec::with_capacity(cap));
    
    for n in list_of_numbers{
        // println!("x: {}, y: {} | x: {}, y: {}",numbers.x1,numbers)
        if n.x1 == n.x2{
            let i = n.x1 as usize;
            if n.y1 > n.y2{
                for j in n.y2..=n.y1{
                    matrix.data[i * matrix.cols + j as usize] +=1;
                }
            } else {
                for j in n.y1..=n.y2{
                    matrix.data[i * matrix.cols + j as usize] +=1;
                }
            }
        }
        if n.y1 == n.y2{
            let j = n.y1 as usize;
            if n.x1 > n.x2{
                for i in n.x2..=n.x1{
                    matrix.data[i as usize * matrix.cols + j] +=1;
                }
            } else {
                for i in n.x1..=n.x2{
                    matrix.data[i as usize * matrix.cols + j] +=1;
                }
            }
        }
    }

    // //sanity
    // matrix.print_matrix();

    let mut count = 0;
    for element in matrix.data{
        if element > 1 {count+=1;}
    }
    count
}


fn solve_part2(input: &Vec<String>) -> i32 {
    let mut width = 0;
    let mut height = 0;

    let mut list_of_numbers: Vec<Numbers> = Vec::new();
    for line in input{
        let mut number_to_add:Vec<i32> = Vec::new();
        for half in line.split(" -> "){
            // println!("{half}");
            for number in half.split(","){
                number_to_add.push(number.parse().expect("Expected a number."))
            }
        }
        increase_maximum(number_to_add[0], &mut width);
        increase_maximum(number_to_add[1], &mut height);
        increase_maximum(number_to_add[2], &mut width);
        increase_maximum(number_to_add[3], &mut height);


        list_of_numbers.push(Numbers{
            x1: number_to_add[0], 
            y1: number_to_add[1],
            x2: number_to_add[2],
            y2: number_to_add[3],
        });
    }
    let cap = (height+1)*(width+1);
    let mut matrix:Matrix = Matrix::new(height as usize +1, width as usize +1, Vec::with_capacity(cap as usize));
    
    for n in list_of_numbers{
        if n.x1 == n.x2{
            let i = n.x1 as usize;
            if n.y1 > n.y2{
                for j in n.y2..=n.y1{
                    matrix.data[i * matrix.cols + j as usize] +=1;
                }
            } else {
                for j in n.y1..=n.y2{
                    matrix.data[i * matrix.cols + j as usize] +=1;
                }
            }
        }else if n.y1 == n.y2{
            let j = n.y1 as usize;
            if n.x1 > n.x2{
                for i in n.x2..=n.x1{
                    matrix.data[i as usize * matrix.cols + j] +=1;
                }
            } else {
                for i in n.x1..=n.x2{
                    matrix.data[i as usize * matrix.cols + j] +=1;
                }
            }
        }else if are_in_diagonal(n.x1,n.y1,n.x2,n.y2){
            let m = (n.y2-n.y1) / (n.x2 - n.x1);
            // diagonal from sw to new
            if n.x1 > n.x2 && n.y1 > n.y2{
                for i in n.x2..=n.x1{
                    matrix.data[i as usize * matrix.cols + ( m*(i-n.x2) + n.y2 ) as usize] += 1;
                }
            }
            // diagonal from se to nw
            else if n.x1 > n.x2 && n.y2 > n.y1{
                for i in n.x2..=n.x1{
                    matrix.data[i as usize * matrix.cols + (m*(i-n.x2) + n.y2) as usize] += 1;
                }
            }
            // diagonal from se to nw
            else if n.x2 > n.x1 && n.y1 > n.y2{
                for i in n.x1..=n.x2{
                    matrix.data[i as usize * matrix.cols + (m*(i-n.x1) + n.y1) as usize] += 1;
                }
            }
            // diagonal from sw to ne
            else if n.x2 > n.x1 && n.y2 > n.y1{
                for i in n.x1..=n.x2{
                    matrix.data[i as usize * matrix.cols + (m*(i-n.x1) + n.y1) as usize] += 1;
                }
            }
            
        }
    }

    // //sanity
    // matrix.print_matrix();

    let mut count = 0;
    for element in matrix.data{
        if element > 1 {count+=1;}
    }
    count
}

fn increase_maximum(candidate: i32, max: &mut i32) {
    if candidate > *max{
        *max = candidate
    }
}

fn are_in_diagonal(x1:i32,y1:i32,x2:i32,y2:i32)->bool{
    return (x1-x2).abs() == (y1-y2).abs()
}


struct Numbers{
    x1:i32,
    x2:i32,
    y1:i32,
    y2:i32,
}


pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i32>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, mut data: Vec<i32>) -> Self {
        for _ in 0..rows*cols{
            data.push(0);
        }
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }
    
    #[allow(dead_code)]
    fn print_matrix(&self){
        println!("gonna print the matrix");
        for j in 0..self.rows{
            for i in 0..self.cols{
                print!("{} ", self.data[i*self.cols+j])
            }
            print!("\n");
        }
    }
}



