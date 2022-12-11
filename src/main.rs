use std::{fs, collections::VecDeque, ops::{Add, Sub, Rem, Div}};

fn main() {
    println!("Hello, world!");

}

#[derive(Debug)]
struct Solution {
    file: &'static str,
    add_register: VecDeque<f64>,
    pixels: String
}

impl Solution {
    
    fn origin(file: &'static str) -> Solution {
        Solution { 
            file,
            add_register: VecDeque::from([0_f64]),
            pixels: String::from("................................................................................................................................................................................................................................................"),
        }
    }

    fn solve(&mut self, file: &str) -> i32 {

        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");
    
        let lines = contents.lines();


        let mut cycle: f64 = 0.0;
        let mut add_register: f64 = 1.0;
        let mut signal_strength: f64 = 0.0;

        for line in lines {
            // cycle += 1;

            if cycle.sub(20.0).rem(40.0) == 0.0 {
                println!("Adding to signal strength: {add_register} on cycle: {cycle}");
                signal_strength += add_register * cycle;
            }

            let mut instructions = line.split_whitespace();
            let instruction: &str = instructions.next().expect("No instruction was provided");
            
            let mut delta = 0;


            if instruction != "noop" {            
                delta = instructions
                    .next()
                    .expect("no amount provided")
                    .parse()
                    .unwrap();
            }

            println!("Cycle: {cycle} Add register: {add_register}");

            match instruction {
                "noop" => {
                    self.add_register.push_back(0_f64);
                    self.fun_name(&mut cycle, &mut signal_strength, &mut add_register);
                    
                },
                "addx" => {
                    self.fun_name(&mut cycle, &mut signal_strength, &mut add_register);
                    println!("Adding to register: {delta}");
                    self.add_register.push_back(delta as f64);
                },
                _ => println!("unsupported instruction"),
            }
        }
    
        println!("{:?}", self.pixels);
        signal_strength as i32
    }

    fn fun_name(&mut self, cycle: &mut f64, signal_strength: &mut f64, add_register: &mut f64) {
        if !self.add_register.is_empty() {
            let value = self.add_register.pop_front().expect("No value found in add register queue");
            if value != 0.0 {
                self.draw_pixel(*cycle, *add_register as i32);
                *cycle = cycle.add(1.0);
                self.draw_pixel(*cycle, *add_register as i32);
                if cycle.sub(20.0).rem(40.0) == 0.0 {
                    println!("Adding to signal strength: {add_register} on cycle: {cycle}");
                    *signal_strength += *add_register * *cycle;
                }
                *add_register += value;
            }
        }
        *cycle = cycle.add(1.0);
        self.draw_pixel(*cycle, *add_register as i32);
    }

    fn draw_pixel(&mut self, cycle: f64, x: i32) {
        let row = cycle.div(40.0).floor() as i32;
        
        let pixel = cycle.rem(40.0) -1.0;

        let pixel_should_be_drawn = pixel == (x - 1).into() || pixel == x.into() || pixel == (x + 1).into();

        if pixel_should_be_drawn {
            let range = (cycle as usize - 1)..cycle as usize;
            self.pixels.replace_range(range, "#");

    
        }
        println!("Drawing pixel on row: {row} and column: {pixel}");

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut solution = Solution {
            file: "src/test-case-input.txt",
            add_register: VecDeque::from([0_f64]),
            pixels: String::from("................................................................................................................................................................................................................................................"),

        };
        let answer = solution.solve(solution.file);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_solution() {
        let mut solution = Solution {
            file: "src/input.txt",
            add_register: VecDeque::from([0_f64]),
            pixels: String::from("................................................................................................................................................................................................................................................"),

        };
        let answer = solution.solve(solution.file);
        println!("{answer}")
    }
}