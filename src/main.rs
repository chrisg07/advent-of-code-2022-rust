#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::{fs, collections::{HashMap, VecDeque, HashSet}, char, cell::RefCell, rc::Rc, borrow::Borrow};

fn main() {
    println!("Hello, world!");
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
  pub id: u32,
  pub value: char,
  pub height: u32,
  pub adjacency_list: Vec<Option<NodeRef>>,
  pub neighbors: Vec<u32>,
  pub visited: bool
}

impl Node {
  fn new(id: u32, value: char, neighbors: Vec<u32>) -> Node {
    Node {
      id,
      value,
      height: Self::get_height(value),
      adjacency_list: Vec::new(),
      neighbors,
      visited: false
    }
  }

  fn get_height(char: char) -> u32 {
    match char {
      'a'..='z' => { char as u32 - 97 },
      'S' => { 0 },
      'E' => { 25 },
      _ => { panic!("Character couldn't be mapped to a height value")}
    }
  }
}

#[derive(Debug)]
struct Solution {
    file: &'static str,
    nodes: HashMap<u32, Node>,
    height: u32,
    width: u32
}

impl Solution {
    fn new(file: &'static str, height: u32, width: u32) -> Solution {
        Solution {
            file,
            nodes: HashMap::new(),
            height,
            width
        }
    }

    fn get_neighbors(&self, index: u32) -> Vec<u32> {
      let mut neighbors = Vec::new();

      // left
      if index % self.width != 0 {
        neighbors.push(index - 1);
      }

      // right
      if index % self.width != self.width - 1 {
        neighbors.push(index + 1);
      }

      // up
      if index >= self.width {
        neighbors.push(index - self.width);
      }

      // down
      if index <= (self.width * self.height) - self.width {
        neighbors.push(index + self.width);
      }

      neighbors
    }

    fn solve(&mut self, file: &str) -> u32 {
        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");

        let lines = contents.lines();
        let mut input = String::from("");
        for line in lines {
          input.push_str(line);
        }

        let mut index = 0;
        for char in input.chars() {
          let neighbors = self.get_neighbors(index);
          let node = Node::new(index, char, neighbors);
          self.nodes.insert(index, node);
          index += 1;
        }

        let start = self.nodes.get_key_value(&0);


        println!("{:?}", self.nodes);
        0
    }

    // fn traverse(nodes: HashMap<u32, NodeRef>, node: NodeRef, distance: u32) -> usize {
    //   let mut visited = HashSet::new();
    //   let mut queue = Vec::new();
    //   queue.push(node);

    //   while let has_node = queue.pop().is_some() {
    //     let cur_node = queue.pop().expect("Unable to find node");
    //     if visited.contains(&Rc::as_ptr(&node)) {
    //       continue
    //     }

    //     visited.insert(Rc::as_ptr(&node));

    //     if cur_node.,borvalue == 'E' {
    //       return visited.len()
    //     }

    //     let neighbors = node.borrow().neighbors;
    //     for id in neighbors {
    //       let neighbor = nodes.get(&id).expect("Expected a neighbor but found none");
    //       queue.push(Rc::clone(neighbor));
    //     }
    //   }



      // node.visited = true;
      // for neighbor in &node.neighbors {
      //   let neighbor_node = self.nodes.get_mut(neighbor).expect("Unable to find node with given index");

      //   if neighbor_node.value == 'E' { return 1 }

      //   let should_traverse = neighbor_node.height <= node.height + 1 && !neighbor_node.visited;
      //   if should_traverse { return self.traverse(&mut neighbor_node.borrow().as_ref()) + 1}

      // }

    //   0
    // }
}

pub fn dfs_with_visited(root: NodeRef, target: i32) -> Option<NodeRef> {
  let mut visited = HashSet::new();
  let mut queue: Vec<NodeRef> = Vec::new();
  queue.push(root);

  while !queue.is_empty() {
      let node = queue.pop().expect("Unable to find node");

      if visited.contains(&Rc::as_ptr(&node)) {
          continue;
      }

      visited.insert(Rc::as_ptr(&node));

      if node.borrow().val == target {
          // return Some(node);
      }

      let items = node.borrow();

      // if let Some(left) = &items.left {
      //     queue.push(Rc::clone(left));
      // }

      // if let Some(right) = &items.right {
      //     queue.push(Rc::clone(right));
      // }
  }

  None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_height() {
      assert_eq!(Node::get_height('a'), 0);
      assert_eq!(Node::get_height('S'), 0);
      assert_eq!(Node::get_height('E'), 25);
      assert_eq!(Node::get_height('y'), 24);
      assert_eq!(Node::get_height('j'), 9);
    }

    #[test]
    fn test_input() {
        let mut solution = Solution::new("src/test-case-input.txt", 5, 8);
        let answer = solution.solve(solution.file);
        assert_eq!(answer, 31);
    }

    #[test]
    fn test_solution() {

        let mut solution = Solution::new("src/input.txt", 41, 161);
        let answer = solution.solve(solution.file);
        println!("{answer}")
    }
}
