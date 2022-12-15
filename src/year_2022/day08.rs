use std::collections::HashSet;


pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1533);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 345744);
}

fn solve_part1(input: &Vec<String>) -> usize {
    let tree_matrix = process_input(input);
    let trees = obtain_frontier_trees_number(&tree_matrix);

    let mut trees_from_sides:Vec<(usize,usize)> = Vec::new();
    obtain_tree_from_sides(&mut trees_from_sides, tree_matrix);

    let mut tree_set:HashSet<(usize,usize)> = HashSet::new();
    add_trees_to_set(trees_from_sides, &mut tree_set);
    
    trees + tree_set.len()
}

fn obtain_tree_from_sides(trees_from_sides: &mut Vec<(usize, usize)>, tree_matrix: Matrix) {
    trees_from_sides.append(&mut trees_from_top(&tree_matrix));
    trees_from_sides.append(&mut trees_from_bot(&tree_matrix));
    trees_from_sides.append(&mut trees_from_left(&tree_matrix));
    trees_from_sides.append(&mut trees_from_right(&tree_matrix));
}

fn add_trees_to_set(trees: Vec<(usize, usize)>, tree_set: &mut HashSet<(usize, usize)>) {
    for tree in trees{
        tree_set.insert(tree);
    }
}

fn obtain_frontier_trees_number(tree_matrix: &Matrix) -> usize {
    let trees: usize = (tree_matrix.cols * 2) + (tree_matrix.rows * 2) - 4;
    trees
}

fn process_input(input: &Vec<String>) -> Matrix {
    let max_i: usize = input.len();
    let max_j: usize = input[0].len();
    let mut tree_matrix: Matrix = Matrix::new(max_i, max_j);
    for (i, line) in input.iter().enumerate(){
        for (j, number) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate(){
            tree_matrix.data[i * tree_matrix.cols + j] = number;         
        }
    
    }
    tree_matrix
}

fn trees_from_top(tree_matrix:&Matrix) -> Vec<(usize,usize)> {
    let mut output_trees: Vec<(usize, usize)> = Vec::new();
    for i in 1..tree_matrix.cols - 1 {
        for j in 1..tree_matrix.rows - 1 {
            let mut visible = true;
            for iold in 0..i {
                if tree_matrix.data[i * tree_matrix.cols + j] <= tree_matrix.data[iold * tree_matrix.cols + j] {
                    visible = false;
                    break
                }
            }
            if visible {
                output_trees.push((i,j));
            }
        }
    }
    output_trees
}

fn trees_from_bot(tree_matrix:&Matrix) -> Vec<(usize,usize)> {
    let mut output_trees: Vec<(usize, usize)> = Vec::new();
    for i in (1..tree_matrix.cols - 1).rev() {
        for j in 1..tree_matrix.rows - 1 {
            let mut visible = true;
            for iold in (i+1..=tree_matrix.cols-1).rev() {
                if tree_matrix.data[i * tree_matrix.cols + j] <= tree_matrix.data[iold * tree_matrix.cols + j] {
                    visible = false;
                    break
                }
            }
            if visible {
                output_trees.push((i,j));
            }
        }
    }
    output_trees
}

fn trees_from_left(tree_matrix:&Matrix) -> Vec<(usize,usize)> {
    let mut output_trees: Vec<(usize, usize)> = Vec::new();
    for i in 1..tree_matrix.cols - 1 {
        for j in 1..tree_matrix.rows - 1 {
            let mut visible = true;
            for jold in 0..j {
                if tree_matrix.data[i * tree_matrix.cols + j] <= tree_matrix.data[i * tree_matrix.cols + jold] {
                    visible = false;
                    break
                }
            }
            if visible {
                output_trees.push((i,j));
            }
        }
    }
    output_trees
}

fn trees_from_right(tree_matrix:&Matrix) -> Vec<(usize,usize)> {
    let mut output_trees: Vec<(usize, usize)> = Vec::new();
    for i in 1..tree_matrix.cols - 1 {
        for j in (1..tree_matrix.rows - 1).rev() {
            let mut visible = true;
            for jold in (j+1..=tree_matrix.rows-1).rev() {
                if tree_matrix.data[i * tree_matrix.cols + j] <= tree_matrix.data[i * tree_matrix.cols + jold] {
                    visible = false;
                    break
                }
            }
            if visible {
                output_trees.push((i,j));
            }
        }
    }
    output_trees
}

fn solve_part2(input: &Vec<String>) -> usize {
    let tree_matrix = process_input(input);

    let mut score = 0;

    for i in 1..tree_matrix.rows{
        for j in 1..tree_matrix.cols{
            let new_score = calculate_score(&tree_matrix, (i,j));
            if new_score > score{
                score = new_score
            }
        }
    }
    score
}

fn trees_on_top(tree_matrix:&Matrix, tree_pos:(usize,usize)) -> usize {
    let mut score = 0;
    for i in (0..tree_pos.0).rev(){
        if tree_matrix.data[i * tree_matrix.cols + tree_pos.1] >= tree_matrix.data[tree_pos.0 * tree_matrix.cols + tree_pos.1]{
            score += 1;
            break;
        }
        score +=1;
    }
    score
}

fn trees_on_bot(tree_matrix:&Matrix, tree_pos:(usize,usize)) -> usize {
    let mut score = 0;
    for i in tree_pos.0 + 1..tree_matrix.rows{
        if tree_matrix.data[i * tree_matrix.cols + tree_pos.1] >= tree_matrix.data[tree_pos.0 * tree_matrix.cols + tree_pos.1]{
            score += 1;
            break;
        }
        score +=1;
    }
    score
}

fn trees_on_left(tree_matrix:&Matrix, tree_pos:(usize,usize)) -> usize {
    let mut score = 0;
    for j in (0..tree_pos.1).rev(){
        if tree_matrix.data[tree_pos.0 * tree_matrix.cols + j] >= tree_matrix.data[tree_pos.0 * tree_matrix.cols + tree_pos.1]{
            score += 1;
            break;
        }
        score +=1;
    }
    score
}

fn trees_on_right(tree_matrix:&Matrix, tree_pos:(usize,usize)) -> usize {
    let mut score = 0;
    for j in tree_pos.1 + 1..tree_matrix.cols{
        if tree_matrix.data[tree_pos.0 * tree_matrix.cols + j] >= tree_matrix.data[tree_pos.0 * tree_matrix.cols + tree_pos.1]{
            score += 1;
            break;
        }
        score +=1;
    }
    score
}

fn calculate_score(tree_matrix: &Matrix, tree_pos:(usize,usize)) -> usize { 
    let mut score = 1;
    score *= trees_on_top(tree_matrix, tree_pos);
    score *= trees_on_bot(tree_matrix, tree_pos);
    score *= trees_on_left(tree_matrix, tree_pos);
    score *= trees_on_right(tree_matrix, tree_pos);
    score
}


#[derive(Clone, PartialEq, Debug)]
struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<u32>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize,) -> Self {
        let data:Vec<u32> = vec![0; cols*rows];
        Self { rows, cols, data }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obtain_frontier_trees_number() {
        // Arrange
        let test_matrix = Matrix::new(5,5);           
        let exp_trees = 16;
        
        // Act
        let test_trees = obtain_frontier_trees_number(&test_matrix);

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

    #[test]
    fn test_trees_from_top() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,1,3,1,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = vec![(1,1),(1,2),(1,3),(2,2)];

        // Act
        let test_trees = trees_from_top(&test_matrix);

        // Assert
        assert_eq!(exp_trees.len(), test_trees.len());
        assert!(exp_trees.contains(&test_trees[0]));
        assert!(exp_trees.contains(&test_trees[1]));
        assert!(exp_trees.contains(&test_trees[2]));
    }

    #[test]
    fn test_trees_from_bot() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,1,3,1,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = vec![(1,1),(1,3),(2,2)];

        // Act
        let test_trees = trees_from_bot(&test_matrix);

        // Assert
        assert_eq!(exp_trees.len(), test_trees.len());
        assert!(exp_trees.contains(&test_trees[0]));
        assert!(exp_trees.contains(&test_trees[1]));
        assert!(exp_trees.contains(&test_trees[2]));
    }

    #[test]
    fn test_trees_from_left() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,2,3,1,1,
                                              1,1,2,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = vec![(1,1),(2,1),(2,2),(3,2)];

        // Act
        let test_trees = trees_from_left(&test_matrix);

        // Assert
        assert_eq!(exp_trees.len(), test_trees.len());
        assert!(exp_trees.contains(&test_trees[0]));
        assert!(exp_trees.contains(&test_trees[1]));
        assert!(exp_trees.contains(&test_trees[2]));
    }

    #[test]
    fn test_trees_from_right() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,2,3,1,1,
                                              1,1,2,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = vec![(1,3),(2,2),(3,2)];

        // Act
        let test_trees = trees_from_right(&test_matrix);

        // Assert
        assert_eq!(exp_trees.len(), test_trees.len());
        assert!(exp_trees.contains(&test_trees[0]));
        assert!(exp_trees.contains(&test_trees[1]));
        assert!(exp_trees.contains(&test_trees[2]));
    }

    #[test]
    fn test_trees_on_top() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,1,3,1,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = 1;

        // Act
        let test_trees = trees_on_top(&test_matrix, (2,3));

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

    #[test]
    fn test_trees_on_bot() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,1,3,1,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = 2;

        // Act
        let test_trees = trees_on_bot(&test_matrix, (2,2));

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

    #[test]
    fn test_trees_on_left() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,3,3,1,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = 1;

        // Act
        let test_trees = trees_on_left(&test_matrix, (2,2));

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

    #[test]
    fn test_trees_on_right() {
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,3,3,3,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = 1;

        // Act
        let test_trees = trees_on_right(&test_matrix, (2,2));

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

    #[test]
    fn test_calculate_score(){
        // Arrange
        let mut test_matrix = Matrix::new(5,5);
        let test_matrix_data = vec![1,1,1,1,1,
                                              1,2,2,2,1,
                                              1,3,3,3,1,
                                              1,1,1,1,1,
                                              1,1,1,1,1,];
        test_matrix.data = test_matrix_data;

        let exp_trees = 4;

        // Act
        let test_trees = calculate_score(&test_matrix, (2,2));

        // Assert
        assert_eq!(exp_trees, test_trees);
    }

}