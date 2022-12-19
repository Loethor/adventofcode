// This file is just a template for any day solution.

use std::{cell::RefCell, rc::Rc, collections::HashMap};

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 1647);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 2447);
}

fn solve_part1(input: &Vec<String>) -> usize {
    let mut nodes_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    let mut connection_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.iter(){
        let processed_line = process_line(line);
        let mut iter = processed_line.split_whitespace();
        let name = iter.next().unwrap();
        let value = iter.next().unwrap().parse().unwrap();
        let node = Node::new(name, value);
        nodes_map.insert(name.to_string(), node);
    }
    for line in input.iter(){
        let processed_line = process_line(line);
        let mut iter = processed_line.split_whitespace();
        let name = iter.next().unwrap();
        let value:i32 = iter.next().unwrap().parse().unwrap();

        // while let Some(connection) = iter.next() { 
        //     nodes_map.get_mut(name).unwrap().borrow_mut().edges.push(nodes_map.get(connection).unwrap().clone());
        // }

    }

    0
}

fn process_line(line: &String) -> String {
    let mut copied_line = line.clone();
    copied_line = copied_line.replace("Valve ", "");
    copied_line = copied_line.replace("has flow rate=", "");
    copied_line = copied_line.replace(",", "");
    copied_line = copied_line.replace(";", "");
    copied_line = copied_line.replace("valve ", "valves ");
    copied_line = copied_line.replace("leads ", "lead ");
    copied_line = copied_line.replace("tunnel ", "tunnels ");
    copied_line = copied_line.replace(" tunnels lead to valves", "");
    copied_line
}

fn solve_part2(input: &Vec<String>) -> usize {
    for line in input.iter(){
        
    }
    0
}

#[derive(Clone, Debug)]

struct Node {
    name: String,
    value: i32,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str, value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            value,
            edges: Vec::new(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        // Arrange

        // Act

        // Assert
    }

    #[test]
    fn test_node() {
        let node1 = Node::new("Node 1", 1);
        let node2 = Node::new("Node 2", 2);

        node1.borrow_mut().edges.push(node2.clone());
        node2.borrow_mut().edges.push(node1.clone());
    
        assert_eq!(node1.borrow().name, "Node 1");
        assert_eq!(node1.borrow().value, 1);
        assert_eq!(node1.borrow().edges.len(), 1);
        assert_eq!(node1.borrow().edges[0].borrow().name, "Node 2");
    
        assert_eq!(node2.borrow().name, "Node 2");
        assert_eq!(node2.borrow().value, 2);
        assert_eq!(node2.borrow().edges.len(), 1);
        assert_eq!(node2.borrow().edges[0].borrow().name, "Node 1");
    }
}