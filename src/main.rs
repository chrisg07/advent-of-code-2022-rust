use std::{fs, collections::HashSet};

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
    rope: Vec<Point>,
    tail_visits: HashSet<Point>,
}

impl Solution {
    
    fn origin(file: &'static str) -> Solution {
        let mut tail_visits = HashSet::new();
        Solution { 
            file,
            rope: vec![
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }
            ],
            tail_visits 
        }
    }

    fn solve(&mut self, file: &str) -> usize {

        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");
    
        let lines = contents.lines();

        
        self.tail_visits.insert(self.rope[9]);

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
    
        println!("Head: {:?}", self.rope[0]);
        println!("Tail: {:?}", self.rope[9]);
        let answer = self.tail_visits.len();
        answer
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "R" => self.rope[0].translate(1, 0),
            "U" => self.rope[0].translate(0, 1),
            "L" => self.rope[0].translate(-1, 0),
            "D" => self.rope[0].translate(0, -1),
            _ => {}
        }
        
        self.move_tail(1);
    }

    fn move_tail(&mut self, index: usize) {
        let x_delta = self.rope[index].x - self.rope[index - 1].x;
        let y_delta = self.rope[index].y - self.rope[index - 1].y;
        match (x_delta, y_delta) {
            // same row
            (-2, 0) => self.rope[index].translate(1, 0),
            (2, 0) => self.rope[index].translate(-1, 0),
            (0, -2) => self.rope[index].translate(0, 1),
            (0, 2) => self.rope[index].translate(0, -1),
            // diagonals
            (1, 2) => self.rope[index].translate(-1, -1),
            (1, -2) => self.rope[index].translate(-1, 1),
            (-1, 2) => self.rope[index].translate(1, -1),
            (-1, -2) => self.rope[index].translate(1, 1),
            (2, 1) => self.rope[index].translate(-1, -1),
            (-2, 1) => self.rope[index].translate(1, -1),
            (2, -1) => self.rope[index].translate(-1, 1),
            (-2, -1) => self.rope[index].translate(1, 1),
            (2, 2) => self.rope[index].translate(-1, -1),
            (-2, 2) => self.rope[index].translate(1, -1),
            (2, -2) => self.rope[index].translate(-1, 1),
            (-2, -2) => self.rope[index].translate(1, 1),
            _ => {}
        }
        
        let rope_moved = (self.rope[index].x - self.rope[index - 1].x).abs() < x_delta.abs() || (self.rope[index].y - self.rope[index - 1].y).abs() < y_delta.abs();
        if rope_moved {
            // println!("moved rope section {index}");
            if index == 9 {
                self.tail_visits.insert(self.rope[index]);
            } else {
                self.move_tail(index + 1);
            }
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
            rope: vec![
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }
            ],
            tail_visits: HashSet::new()
        };
        let answer = solution.solve(solution.file);
        assert_eq!(answer, 36);
    }

    #[test]
    fn test_solution() {
        let mut solution = Solution {
            file: "src/input.txt",
            rope: vec![
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }, 
                Point { x: 0, y:0 }
            ],
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