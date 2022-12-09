use std::{fs, borrow::Borrow, collections::HashSet};

fn main() {
    println!("Hello, world!");

}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug)]
struct Solution {
    file: &'static str,
    head: Point,
    tail: Point,
    tail_visits: HashSet<Point>,
}

impl Solution {
    
    fn origin(file: &'static str) -> Solution {
        let mut tail_visits = HashSet::new();
        Solution { file, head: Point { x: 0, y:0 }, tail: Point { x: 0, y:0 }, tail_visits }
    }

    fn solve(&mut self, file: &str) -> usize {

        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");
    
        let lines = contents.lines();

        for line in lines {
            let mut instructions = line.split_whitespace();
            let direction: &str = instructions.next().expect("No direction was provided");
            let delta: i32 = instructions
                .next()
                .expect("No delta value was provided")
                .parse()
                .unwrap();
            
            for _step in 0..delta {
                self.move_head(direction);
            }
        }
    
        println!("Head: {:?}", self.head);
        println!("Tail: {:?}", self.tail);
        let answer = self.tail_visits.len();
        answer
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "R" => self.head.translate(1, 0),
            "U" => self.head.translate(0, 1),
            "L" => self.head.translate(-1, 0),
            "D" => self.head.translate(0, -1),
            _ => {}
        }
        
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let x_delta = self.tail.x - self.head.x;
        let y_delta = self.tail.y - self.head.y;
        match (x_delta, y_delta) {
            // same row
            (-2, 0) => self.tail.translate(1, 0),
            (2, 0) => self.tail.translate(-1, 0),
            (0, -2) => self.tail.translate(0, 1),
            (0, 2) => self.tail.translate(0, -1),
            // diagonals
            (1, 2) => self.tail.translate(-1, -1),
            (1, -2) => self.tail.translate(-1, 1),
            (-1, 2) => self.tail.translate(1, -1),
            (-1, -2) => self.tail.translate(1, 1),
            (2, 1) => self.tail.translate(-1, -1),
            (-2, 1) => self.tail.translate(1, -1),
            (2, -1) => self.tail.translate(-1, 1),
            (-2, -1) => self.tail.translate(1, 1),
            _ => {}
        }
        
        if (self.tail.x - self.head.x).abs() < x_delta.abs() || (self.tail.y - self.head.y).abs() < y_delta.abs() {
            self.tail_visits.insert(self.tail);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut solution = Solution {
            file: "src/test-case-input.txt",
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_visits: HashSet::new()
        };
        let answer = solution.solve(solution.file);
        assert_eq!(solution.head.x, 2);
        assert_eq!(solution.head.y, 2);
        assert_eq!(solution.tail.x, 1);
        assert_eq!(solution.tail.y, 2);
        assert_eq!(answer, 13);
    }

    #[test]
    fn test_solution() {
        let mut solution = Solution {
            file: "src/input.txt",
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_visits: HashSet::new()
        };
        let answer = solution.solve(solution.file);
        println!("{answer}")
    }

    #[test]
    fn test_translate() {
        let mut point = Point { x: 0, y: 0 };
        point.translate(1, 1);
        assert_eq!(point.x, 1);
        assert_eq!(point.x, 1);
    }
}