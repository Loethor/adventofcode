use std::ops::Add;
use std::iter::zip;

pub fn run(input: Vec<String>) {

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 631);


    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!("EFLFJGRF", "EFLFJGRF");
}

fn solve_part1(input: &Vec<String>) -> i32 {
    let mut rows = 0;
    let mut cols = 0;
    let mut coordinates:Vec<(usize,usize)> = Vec::new();
    let mut folds:Vec<(char,usize,usize)> = Vec::new();

    let mut fold_count = 0;
    for line in input{
        if line.len() == 0{continue;}
        if line.contains(','){
            let (j,i) = line.split_once(',').unwrap();
            let i:usize = i.parse().unwrap();
            let j:usize = j.parse().unwrap();
    
            if i+1>rows{rows=i+1;}
            if j+1>cols{cols=j+1;}
            coordinates.push((i,j));
        } else {
            let (i,j) = line.split_once('=').unwrap();
            if i.contains('y'){
                folds.push(('y',fold_count,j.parse().unwrap()))
            } else if i.contains('x') {
                folds.push(('x',fold_count,j.parse().unwrap()))
            }
            fold_count += 1;
        }
    }

    let mut matrix = Matrix::new(rows, cols);
    for element in coordinates{
        matrix.data[element.0 * matrix.cols + element.1] = 1;
    }

    for fold in folds{
        if fold.0 == 'x'{
            matrix = matrix.fold_x(fold.2);
        } else if fold.0 == 'y'{
            matrix = matrix.fold_y(fold.2);
        }
        if fold.1 == 0{
            break;
        }
    }
    matrix.data.iter().sum()
}

fn solve_part2(input: &Vec<String>) -> i32 {
    let mut rows = 0;
    let mut cols = 0;
    let mut coordinates:Vec<(usize,usize)> = Vec::new();
    let mut folds:Vec<(char,usize,usize)> = Vec::new();

    let mut fold_count = 0;
    for line in input{
        if line.len() == 0{continue;}
        if line.contains(','){
            let (j,i) = line.split_once(',').unwrap();
            let i:usize = i.parse().unwrap();
            let j:usize = j.parse().unwrap();
    
            if i+1>rows{rows=i+1;}
            if j+1>cols{cols=j+1;}
            coordinates.push((i,j));
        } else {
            let (i,j) = line.split_once('=').unwrap();
            if i.contains('y'){
                folds.push(('y',fold_count,j.parse().unwrap()))
            } else if i.contains('x') {
                folds.push(('x',fold_count,j.parse().unwrap()))
            }
            fold_count += 1;
        }
    }

    let mut matrix = Matrix::new(rows, cols);
    for element in coordinates{
        matrix.data[element.0 * matrix.cols + element.1] = 1;
    }

    for fold in folds{
        if fold.0 == 'x'{
            matrix = matrix.fold_x(fold.2);
        } else if fold.0 == 'y'{
            matrix = matrix.fold_y(fold.2);
        }
    }
    matrix.print_matrix();
    matrix.data.iter().sum()
}

#[derive(Clone, PartialEq, Debug)]
struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<i32>,
}

impl Matrix {

    pub fn new(rows: usize, cols: usize,) -> Self {
        let data:Vec<i32> = vec![0; cols*rows];
        Self { rows, cols, data }
    }

    fn mirror_in_x(&mut self){
        let matrix_copy = self.clone();
        for i in 0..self.rows{
            for j in 0..self.cols{
                self.data[(i)*self.cols+j] = matrix_copy.data[(i)*self.cols+self.cols-1-j]
            }
        }
    }

    fn mirror_in_y(&mut self){
        let matrix_copy = self.clone();
        for i in 0..self.rows{
            for j in 0..self.cols{
                self.data[(i)*self.cols+j] = matrix_copy.data[(self.rows-1-i)*self.cols+j]
            }
        }
    }

    fn reduce_matrix(&self, new_rows:usize, new_cols:usize) -> Matrix{
        let mut new_matrix = Matrix::new(new_rows, new_cols);
        for i in 0..new_rows{
            for j in 0..new_cols{
                new_matrix.data[i*new_matrix.cols+j] = self.data[i*self.cols+j]
            }
        }
        new_matrix
    }
    
    fn clip_matrix_to_one(&mut self){
        for element in &mut self.data{
            if element > &mut 0{
                *element = 1;
            }
        }
    }

    fn fold_x(self:&Matrix, fold:usize) -> Matrix{
        let copied_matrix:Matrix = self.clone();
        let mut x_mirror_matrix:Matrix = self.clone();
        x_mirror_matrix.mirror_in_x();
        x_mirror_matrix = copied_matrix + x_mirror_matrix;
        let new_cols = self.cols - (fold + 1);
    
        x_mirror_matrix = x_mirror_matrix.reduce_matrix(self.rows, new_cols);
        x_mirror_matrix.clip_matrix_to_one();
        x_mirror_matrix
    }

    fn fold_y(self:&Matrix, fold:usize) -> Matrix{
        let copied_matrix:Matrix = self.clone();
        let mut y_mirror_matrix:Matrix = self.clone();
        y_mirror_matrix.mirror_in_y();
        y_mirror_matrix = copied_matrix + y_mirror_matrix;
        let new_rows = self.rows - (fold + 1);
        y_mirror_matrix = y_mirror_matrix.reduce_matrix(new_rows,self.cols);
        y_mirror_matrix.clip_matrix_to_one();
        y_mirror_matrix
    }
    #[allow(dead_code)]
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
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut add_data:Vec<i32> = Vec::new();
        let iter = zip(self.data, other.data);
        for (a,b) in iter{
            add_data.push(a+b);
        }
        Self {
            rows: self.rows,
            cols: self.cols,
            data: add_data
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_on_x() {
        let mut t_matrix = Matrix{rows:2,cols:2, data:vec![1,2,3,4]};
        let e_matrix = Matrix{rows:2,cols:2, data:vec![2,1,4,3]};
        t_matrix.mirror_in_x();
        assert_eq!(t_matrix, e_matrix);
    }

    #[test]
    fn test_mirror_on_y() {
        let mut t_matrix = Matrix{rows:2,cols:2, data:vec![1,2,3,4]};
        let e_matrix = Matrix{rows:2,cols:2, data:vec![3,4,1,2]};
        t_matrix.mirror_in_y();
        assert_eq!(t_matrix, e_matrix);
    }

    #[test]
    fn test_reduce_matrix() {
        let mut t_matrix = Matrix{rows:3,cols:3, data:vec![1,2,3,4,5,6,7,8,9]};
        let e_matrix = Matrix{rows:2,cols:2, data:vec![1,2,4,5]};
        let mut t_matrix_2 = Matrix{rows:3,cols:3, data:vec![1,2,3,4,5,6,7,8,9]};
        let e_matrix_2 = Matrix{rows:1,cols:2, data:vec![1,2]};
        let mut t_matrix_3 = Matrix{rows:3,cols:3, data:vec![1,2,3,4,5,6,7,8,9]};
        let e_matrix_3 = Matrix{rows:2,cols:1, data:vec![1,4]};


        t_matrix = t_matrix.reduce_matrix(2,2);
        t_matrix_2 = t_matrix_2.reduce_matrix(1,2);
        t_matrix_3 = t_matrix_3.reduce_matrix(2,1);

        assert_eq!(t_matrix, e_matrix);
        assert_eq!(t_matrix_2, e_matrix_2);
        assert_eq!(t_matrix_3, e_matrix_3);
    }

    #[test]
    fn test_add_matrices(){
        let t_matrix = Matrix{rows:2,cols:2, data:vec![1,2,3,4]};
        let t_matrix_2 = Matrix{rows:2,cols:2, data:vec![1,2,3,4]};
        let e_matrix = Matrix{rows:2,cols:2, data:vec![2,4,6,8]};
        let r_matrix = t_matrix + t_matrix_2;
        assert_eq!(r_matrix, e_matrix);

    }

    #[test]
    fn test_matrix_clipped_to_one(){
        let mut t_matrix = Matrix{rows:2,cols:2, data:vec![1,2,3,0]};
        let e_matrix = Matrix{rows:2,cols:2, data:vec![1,1,1,0]};

        t_matrix.clip_matrix_to_one();
        assert_eq!(t_matrix, e_matrix);
    }

    #[test]
    fn test_fold_y(){
        let mut t_matrix = Matrix{rows:5,cols:5, data:vec![1,0,1,0,0,
                                                                     0,0,0,0,1,
                                                                     0,0,0,0,0,
                                                                     0,0,0,0,0,
                                                                     1,1,1,1,1,]};
        let mut e_matrix = Matrix{rows:2,cols:5, data:vec![1,1,1,1,1,
                                                                     0,0,0,0,1,]};
        let fold = 2;
        let r_matrix = t_matrix.fold_y(fold);
        assert_eq!(r_matrix.rows, t_matrix.rows - (fold + 1));
        // assert_eq!(r_matrix.cols, t_matrix.cols);
        assert_eq!(r_matrix, e_matrix);
    }

    #[test]
    fn test_fold_x(){
        let mut t_matrix = Matrix{rows:5,cols:5, data:vec![1,0,1,0,0,
                                                                     0,0,0,0,1,
                                                                     0,0,0,0,0,
                                                                     0,0,0,0,0,
                                                                     1,1,1,1,1,]};
        let mut e_matrix = Matrix{rows:5,cols:2, data:vec![1,0,
                                                                     1,0,
                                                                     0,0,
                                                                     0,0,
                                                                     1,1]};
        let fold = 2;
        let r_matrix = t_matrix.fold_x(fold);
        assert_eq!(r_matrix.cols, t_matrix.cols - (fold + 1));
        // assert_eq!(r_matrix.cols, t_matrix.cols);
        assert_eq!(r_matrix, e_matrix);
    }
}
