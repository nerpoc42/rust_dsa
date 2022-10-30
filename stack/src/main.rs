extern crate stack;
use stack::*;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    println!("{}", stack.peek().map_or_else(|| String::from("No number"), i32::to_string));

    println!("{}", stack.contains(&19));

    while let Some(value) = stack.pop() {
        println!("{}", value)
    }
}
