// This file is just a template for any day solution.
use std::time::{Instant};

pub fn run(input: Vec<String>) {
    let start = Instant::now();
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 8372);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());

    let start = Instant::now();
    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 7865110481723);
    println!("Time elapsed in part1 is: {:?}", start.elapsed());
}

fn solve_part1(input: &Vec<String>) -> i64 {
    
    let rounds:i64 = 1;
    let key:i64 = 1;

    let decryption_ring:Vec<i64> = input.iter().map(|n| n.parse().unwrap()).collect();
    let mut indices :Vec<i64>= (0..(decryption_ring.len() as i64)).collect();

    mix(&decryption_ring, &mut indices, rounds, key);
    let coord_sum = find_sum(&decryption_ring, &indices, key);
    coord_sum
}
fn mix(decryption_ring: &Vec<i64>, indices: &mut Vec<i64>, rounds:i64, key:i64) {
    for _ in 0..rounds {
        for i in 0..(decryption_ring.len() as i64) {
            let n = indices.iter().position(|&x| x == i).unwrap();
            indices.remove(n);
            let new_position:i64 = (n as i64) + decryption_ring[i as usize] * key;
            let new_position = new_position.rem_euclid(indices.len() as i64);
            indices.insert(new_position as usize, i);
        }
    }
}

fn find_sum(decryption_ring: &Vec<i64>, indices: &Vec<i64>, key:i64) -> i64 {
    let zero_index = decryption_ring.iter().position(|&x| x == 0).unwrap();
    let zero = indices.iter().position(|&x| x == zero_index as i64).unwrap();
    return [1000, 2000, 3000].iter()
        .map(|x| decryption_ring[indices[(zero + x) as usize % indices.len()] as usize] * key).sum();
}

fn solve_part2(input: &Vec<String>) -> i64 {
    let rounds:i64 = 10;
    let key:i64 = 811589153;
    
    let decryption_ring:Vec<i64> = input.iter().map(|n| n.parse().unwrap()).collect();
    let mut indices :Vec<i64>= (0..(decryption_ring.len() as i64)).collect();

    mix(&decryption_ring, &mut indices, rounds, key);
    let coord_sum = find_sum(&decryption_ring, &indices, key);
    coord_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example(){
        let test_ring:Vec<i64> = Vec::from([1, 2, -3, 3, -2, 0, 4]);
        let mut test_indices:Vec<i64> = (0..test_ring.len() as i64).collect();

        mix(&test_ring, &mut test_indices, 1, 1);
        let test_sum = find_sum(&test_ring, &test_indices, 1);

        assert_eq!(test_sum, 3);
    }
}