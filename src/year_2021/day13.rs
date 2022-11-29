pub fn run(input: Vec<String>) {

    let part1 = solve_part1b(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1702);


    // let part2 = solve_part2(&input);
    // println!("part 2: {}", part2);
    // // assert_eq!(part2, 94813675);
}
fn solve_part1(input: &Vec<String>) -> usize {
    let mut m = 0;
    let mut n = 0;
    let mut coordinates:Vec<(usize,usize)> = Vec::new();
    for line in &input[0..18]{
        let (j,i) = line.split_once(',').unwrap();
        let i:usize = i.parse().unwrap();
        let j:usize = j.parse().unwrap();

        if i+1>m{m=i+1;}
        if j+1>n{n=j+1;}
        println!("{} {}",i,j);
        coordinates.push((i,j));
    }
    println!("{} {}",m,n);
    let mut matrix:Vec<char> = Vec::new();
    for i in 0..m{
        for j in 0..n{
            matrix.push('.');
        }
    }
    for element in coordinates{
        matrix[element.0*n+element.1] = '#';
    }

    for i in 0..m{
        for j in 0..n{
            print!("{} ",matrix[i*n+j] )
        }
        println!("")
    }
    let mut count = 0;
    for i in 0..m{
        for j in 0..n{
            if matrix[i*n+j] == '#'{count+=1;}
        }
        println!("")
    }
    println!("{count}");

    let mut matrix_a:Vec<char> = Vec::new();
    let mut matrix_b:Vec<char> = Vec::new();

    let fold_i=7;
    let fold_j=5;

    println!("a");
    for i in 0..m-fold_i-1{
        for j in 0..n{
            matrix_a.push(matrix[i*n+j])
        };
    }

    for i in 0..m-fold_i-1{
        for j in 0..n{
            print!("{} ",matrix_a[i*n+j] )
        }
        println!("")
    }

    println!("b");

    for i in fold_i+1..m{
        for j in 0..n{
            matrix_b.push(matrix[i*n+j])
        };
    }

    for i in 0..m-fold_i-1{
        for j in 0..n{
            print!("{} ",matrix_b[(fold_i-i-1)*n+j] )
        }
        println!("")
    }

    let mut matrix_c:Vec<char> = Vec::new();

    for i in 0..m-fold_i{
        for j in 0..n{

            if matrix_a[i*n+j] == '#' || matrix_b[(fold_i-i)*n+j] == '#'{
                matrix_c.push('#');
            } else {
                matrix_c.push('.');
            }
        }
    }

    println!("c");
    for i in 0..m-fold_i{
        for j in 0..n{
            print!("{} ",matrix_c[(i)*n+j] )
        }
        println!("")
    }


    let mut count = 0;
    for i in 0..m-fold_i{
        for j in 0..n{
            if matrix_c[i*n+j] == '#'{count+=1;}
        }
    }
    println!("{count}");


    println!("a2");
    for i in 0..m-fold_i{
        for j in 0..n-fold_j{
            matrix_a.push(matrix_c[i*(n-fold_j)+j])
        };
    }
    for i in 0..m-fold_i{
        for j in 0..n-fold_j{
            print!("{} ",matrix_a[i*(n-fold_j)+j] )
        }
        println!("")
    }


    println!("b2");
    for i in 0..m-fold_i{
        for j in fold_j..n{
            matrix_b.push(matrix_c[i*(n-fold_j)+j])
        };
    }
    for i in 0..m-fold_i{
        for j in 0..n-fold_j{
            print!("{} ",matrix_b[i*(n-fold_j)+fold_j-j] )
        }
        println!("")
    }


    for i in 0..m-fold_i{
        for j in 0..n-fold_j{

            if matrix_a[i*(n-fold_j)+j] == '#' || matrix_b[i*(n-fold_j)+fold_j-j] == '#'{
                matrix_c.push('#');
            } else {
                matrix_c.push('.');
            }
        }
    }






    println!("c2");
    for i in 0..m-fold_i{
        for j in 0..n-fold_j{
            print!("{} ",matrix_c[(i)*(n-fold_j)+j] )
        }
        println!("")
    }


    let mut count = 0;
    for i in 0..m-fold_i{
        for j in 0..n-fold_j{
            if matrix_c[i*(n-fold_j)+j] == '#'{count+=1;}
        }
        println!("")
    }
    println!("{count}");

    
    
    0
}

fn solve_part2(input: &Vec<String>) -> usize {
    0
}


fn solve_part1b(input: &Vec<String>) -> usize {

    let mut m = 0;
    let mut n = 0;
    let mut coordinates:Vec<(usize,usize)> = Vec::new();
    let mut folds:Vec<(char,usize)> = Vec::new();

    for line in input{
        if line.len() == 0{continue;}
        if line.contains(','){
            let (j,i) = line.split_once(',').unwrap();
            let i:usize = i.parse().unwrap();
            let j:usize = j.parse().unwrap();
    
            if i+1>m{m=i+1;}
            if j+1>n{n=j+1;}
            println!("{} {}",i,j);
            coordinates.push((i,j));
        } else{
            let (i,j) = line.split_once('=').unwrap();
            if i.contains('y'){
                folds.push(('y',j.parse().unwrap()))
            } else if i.contains('x') {
                folds.push(('x',j.parse().unwrap()))
            }
        }
    }
    let mut matrix = Matrix::new(m, n);
    println!("limints {} {}",m,n);

    for element in coordinates{
        matrix.data[element.0*n+element.1] = '#';
    }

    // println!("{} {}", folds[0].0,folds[1].0);
    // println!("{} {}", folds[0].1,folds[1].1);
    // let fold_i=7;
    let fold_i=folds[0].1;
    // let fold_j=5;
    let fold_j=folds[1].1;

    let mut n_folds = 0;
    for fold in &folds{
        if fold.0 == 'x'{
            println!("{} {}", fold.0,fold.1);
            (m,n, matrix) = fold_on_x(m, n, fold.1, &matrix);
            println!("Folded on x: {}", fold.1)
        }

        if fold.0 == 'y'{
            println!("{} {}", fold.0,fold.1);
            (m,n, matrix) = fold_on_y(m, n, fold.1, &matrix);
            println!("Folded on y: {}", fold.1)
        }

        println!("Number of # {}", matrix.count());

        n_folds += 1;
        // if n_folds > 0{
        //     break;
        // }
    }


    // let (m,n, matrix) = fold_on_y(m,n, fold_i,  &matrix);
    // println!("After first fold {}",matrix.count());
    // // matrix_c.print_matrix();

    // let (m,n, matrix) = fold_on_x(m, n, fold_j, &matrix);
    matrix.print_matrix();









    matrix.count() as usize
}

fn fold_on_x( m: usize, n: usize, fold_j: usize, matrix: &Matrix) -> (usize, usize, Matrix) {
    let n_b = n-fold_j;
    let (matrix_a, mut matrix_b) = split_matrix_on_x(m, n_b, matrix, matrix.cols);
    // matrix_b.print_matrix();
    matrix_b.mirror_though_x_fold();
    // matrix_b.print_matrix();
    let mut matrix_c = Matrix::new(m-1, n_b-1);
    matrix_c.union_2(&matrix_a, &matrix_b);
    (m, n_b, matrix_c)
}

fn split_matrix_on_x(m_a: usize, n_b: usize, matrix: &Matrix, n: usize) -> (Matrix, Matrix) {
    let mut matrix_a = Matrix::new(m_a-1, n_b-1);
    for i in 0..m_a-1{
        for j in 0..n_b-1{
            matrix_a.data[i*(n_b-1)+j]=matrix.data[i*(n)+j]
        }
    }
    // matrix_a.print_matrix();
    let mut matrix_b = Matrix::new(m_a-1, n_b-1);
    for i in 0..m_a-1{
        for j in n_b..n{
            matrix_b.data[(i)*(n_b-1)+j-n_b]=matrix.data[i*(n)+j];
        }
    }
    (matrix_a, matrix_b)
}

fn fold_on_y(m: usize, n: usize, fold_i: usize,  matrix: &Matrix) -> (usize, usize, Matrix) {
    let m_a = m-fold_i;
    let (matrix_a, mut matrix_b) = split_matrix_on_y(m_a, n, matrix, matrix.rows);
    // matrix_b.print_matrix();
    matrix_b.mirror_though_y_fold();
    // matrix_b.print_matrix();
    // matrix_a.union(&matrix_b);
    // matrix_a.print_matrix();
    let mut matrix_c = Matrix::new(m_a-1, n);
    matrix_c.union_2(&matrix_a, &matrix_b);
    (m_a, n, matrix_c)
}

fn split_matrix_on_y(m_a: usize, n: usize, matrix: &Matrix, m: usize) -> (Matrix, Matrix) {
    let mut matrix_a = Matrix::new(m_a-1, n);
    for i in 0..m_a-1{
        for j in 0..n{
            matrix_a.data[i*n+j]=matrix.data[i*n+j]
        }
    }
    // matrix_a.print_matrix();
    let mut matrix_b = Matrix::new(m_a-1, n);
    for i in m_a..m{
        for j in 0..n{
            matrix_b.data[(i-m_a)*n+j] = matrix.data[i*n+j]
        }
    }
    (matrix_a, matrix_b)
}

#[derive(Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}





impl Matrix {
    pub fn new(rows: usize, cols: usize,) -> Self {
        let data:Vec<char> = vec!['.'; cols*rows];
        Self { rows, cols, data }
    }

    fn count(&self) -> i32{
        let mut count = 0;
        for i in 0..self.rows{
            for j in 0..self.cols{
                if self.data[i*self.cols+j] == '#' {
                    count +=1;
                }
                }
            }
        count
    }
    

    fn union(&mut self, other: &Matrix){
        for i in 0..self.rows{
            for j in 0..self.cols{
                if self.data[i*self.cols+j] == '#' || other.data[i*self.cols+j] == '#'{
                    self.data[i*self.cols+j] = '#';
                }
            }
        }
    }

    fn union_2(&mut self, matrix_a:&Matrix, matrix_b: &Matrix){
        for i in 0..self.rows{
            for j in 0..self.cols{
                if matrix_a.data[i*self.cols+j] == '#' || matrix_b.data[i*self.cols+j] == '#'{
                    self.data[i*self.cols+j] = '#';
                }
            }
        }
    }
    
    // #[allow(dead_code)]
    fn print_matrix(&self){
        println!("");
        for i in 0..self.rows{
            for j in 0..self.cols{
                print!("{} ", self.data[i*self.cols+j])
            }
            print!("\n");
        }
        println!("");
    }

    fn mirror_though_y_fold(&mut self){
        let matrix_copy = self.clone();
        for i in 0..self.rows{
            for j in 0..self.cols{
                self.data[(i)*self.cols+j] = matrix_copy.data[(self.rows-1-i)*self.cols+j]
            }
        }
    }

    fn mirror_though_x_fold(&mut self){
        let matrix_copy = self.clone();
        for i in 0..self.rows{
            for j in 0..self.cols{
                self.data[(i)*self.cols+j] = matrix_copy.data[(i)*self.cols+self.cols-1-j]
            }
        }
    }


}
