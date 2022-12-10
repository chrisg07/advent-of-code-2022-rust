use std::{fs, collections::{HashSet, VecDeque}};

fn main() {
    println!("Hello, world!");

}

#[derive(Debug)]
struct Solution {
    file: &'static str,
    add_register: VecDeque<i32>
}

impl Solution {
    
    fn origin(file: &'static str) -> Solution {
        Solution { 
            file,
            add_register: VecDeque::from([0])
        }
    }

    fn solve(&mut self, file: &str) -> i32 {

        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");
    
        let lines = contents.lines();


        let mut cycle: i32 = 0;
        let mut add_register = 1;
        let mut signal_strength = 0;

        for line in lines {
            // cycle += 1;

            if (cycle - 20) % 40 == 0 {
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
                    self.add_register.push_back(0);
                    if !self.add_register.is_empty() {
                        let value = self.add_register.pop_front().expect("No value found in add register queue");
                        if value != 0 {
                            cycle += 1;
                            
                            if (cycle - 20) % 40 == 0 {
                                println!("Adding to signal strength: {add_register} on cycle: {cycle} noop");
                                signal_strength += add_register * cycle;
                            }
                            add_register += value;
                        }
                    }
                    cycle += 1;
                    
                },
                "addx" => {
                    if !self.add_register.is_empty() {
                        let value = self.add_register.pop_front().expect("No value found in add register queue");
                        if value != 0 {
                            cycle += 1;
                            
                            if (cycle - 20) % 40 == 0 {
                                println!("Adding to signal strength: {add_register} on cycle: {cycle}");
                                signal_strength += add_register * cycle;
                            }
                            add_register += value;
                        }
                    }
                    println!("Adding to register: {delta}");
                    self.add_register.push_back(delta);
                    cycle += 1;
                },
                _ => println!("unsupported instruction"),
            }
        }
    
        signal_strength
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut solution = Solution {
            file: "src/test-case-input.txt",
            add_register: VecDeque::from([0])
        };
        let answer = solution.solve(solution.file);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_solution() {
        let mut solution = Solution {
            file: "src/input.txt",
            add_register: VecDeque::from([0])
        };
        let answer = solution.solve(solution.file);
        println!("{answer}")
    }
}