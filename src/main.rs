#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::{fs};

fn main() {
    println!("Hello, world!");

}
#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<u32>,
  operation: Vec<String>,
  division_test: u32,
  true_target: u8,
  false_target: u8,
  identifier: u8,
  interactions: u32
}

impl Monkey {
  fn build(identifier: u8) -> Monkey {
    Monkey {
      items: Vec::new(),
      operation: Vec::new(),
      division_test: 0,
      true_target: 0,
      false_target: 0,
      identifier: identifier,
      interactions: 0
    }
  }
}

#[derive(Debug, Clone)]
struct MonkeyList {
  monkeys: Vec<Monkey>
}

impl MonkeyList {
  fn build() -> MonkeyList {
    MonkeyList {
      monkeys: Vec::<Monkey>::new()
    }
  }

  fn add_monkey(&mut self, monkey: Monkey) {
    self.monkeys.push(monkey);
  }
}

#[derive(Debug)]
struct Solution {
    file: &'static str,
    monkeys: MonkeyList
}

impl Solution {

    fn origin(file: &'static str) -> Solution {
        Solution {
            file,
            monkeys: MonkeyList::build(),
        }
    }

    fn solve(&mut self, file: &str) -> u32 {
        let contents = fs::read_to_string(file)
            .expect("Should have been able to read the file");

        let lines = contents.lines();

        let mut monkey_identifier = 0;
        let mut monkey = Monkey::build(monkey_identifier);

        for line in lines {
            let mut instructions = line.split_whitespace();
            let first_instruction = instructions.next();
            if first_instruction.is_some() {
              let value = first_instruction.expect("No instruction");
              match value {
                "Monkey" => {
                  monkey = Monkey::build(monkey_identifier);
                  monkey_identifier += 1;
                },
                "Starting" => {
                  instructions.next().expect("Nothing to throw away");
                  for item_str in instructions {
                    if item_str.ends_with(",") {
                      let item_num = &item_str[0..item_str.len() - 1];
                      let item = item_num.parse().expect("Unable to convert to integer");
                      monkey.items.push(item);
                    } else {
                      let item = item_str.parse().expect("Unable to convert string to integer");
                      monkey.items.push(item);
                    }
                  }
                },
                "Operation:" => {
                  instructions.next().expect("Nothing to throw away");
                  instructions.next().expect("Nothing to throw away");
                  for item_str in instructions {
                    monkey.operation.push(String::from(item_str));
                  }
                },
                "Test:" => {
                  let test_value = instructions.last().expect("Unable to find the last value");
                  monkey.division_test = test_value.parse().expect("Unable to parse division test value");
                },
                "If" => {
                  let second_instruction = instructions.next().expect("No instruction found");
                  match second_instruction {
                    "true:" => {
                      let target = instructions.last().expect("Unable to find the last value");
                      monkey.true_target = target.parse().expect("Unable to parse true target");
                    },
                    "false:" => {
                      let target = instructions.last().expect("Unable to find the last value");
                      monkey.false_target = target.parse().expect("Unable to parse true target");
                    },
                    _ => {}
                  }
                }
                _ => {}
              }
            } else {
              println!("Added monkey: {:?}", monkey);
              self.monkeys.add_monkey(monkey.clone());
            }
        }

        println!("Added monkey: {:?}", monkey);
        self.monkeys.add_monkey(monkey.clone());

        let mut monkes = Self::take_turns(self.monkeys.monkeys.clone(), 20);
        monkes.sort_by(|a, b| b.interactions.cmp(&a.interactions));
        println!("{:?}", monkes);

        monkes[0].interactions * monkes[1].interactions
    }

    fn take_turns(mut monkeys: Vec<Monkey>, rounds: usize) -> Vec<Monkey> {
      for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
          let monkey = &mut monkeys[monkey_index];
          println!("Monkey {}:", monkey.identifier);
          let items: Vec<_> = monkey.items
            .drain(..)
            .map(|item| {
              println!("  Monkey inspects an item with a worry level of {}.", item);

              let mut worry_level = item;
              let left_operation = monkey.operation.get(0).expect("No left hand operation");
              let mut left = worry_level;
              if left_operation != "old" { left = left_operation.parse().expect("Unable to parse left hand value") }

              let right_operation = monkey.operation.get(2).expect("No left hand operation");
              let mut right = worry_level;
              if right_operation != "old" { right = right_operation.parse().expect("Unable to parse left hand value") }

              let operator = monkey.operation.get(1).expect("No operator found");
              match operator.as_str() {
                "*" => { worry_level = left * right },
                "+" => { worry_level = left + right },
                _ => {}
              }
              println!("    Worry level is now {}", worry_level);

              worry_level = worry_level / 3;
              println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
              monkey.interactions += 1;
              let mut target: usize = 0;
              if worry_level % monkey.division_test == 0 {
                println!("    Current worry level is divisible by {}.", monkey.division_test);
                target = monkey.true_target as usize;
              } else {
                println!("    Current worry level is not divisible by {}.", monkey.division_test);
                target = monkey.false_target as usize;

              }
              (worry_level, target)
            })
            .collect();


          for (item, target) in items {
            println!("    Item with worry level {item} is thrown to monkey {}.", target);
            monkeys[target].items.push(item);
          }
        }
      }
      monkeys
  }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut solution = Solution::origin("src/test-case-input.txt");
        let answer = solution.solve(solution.file);
        assert_eq!(solution.monkeys.monkeys.len(), 4);
        assert_eq!(answer, 10605);
    }

    #[test]
    fn test_solution() {

        let mut solution = Solution::origin("src/input.txt");
        let answer = solution.solve(solution.file);
        println!("{answer}")
    }
}
