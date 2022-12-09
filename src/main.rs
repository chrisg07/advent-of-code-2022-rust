use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("src/test-case-input.txt")
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    for line in lines {
        for char in line.chars() {

            println!("{}", char)
        }
    }
    println!("With text:\n{contents}");
}

#[test]
fn it_works() {
    assert_eq!(4, 4);
}