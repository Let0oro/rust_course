//! The lifetime of a value refers to how long is valid within the code
//!
//! A value's lifetime is the time during which it exists at a particular memory address

mod common;
mod elision;
mod generic;

pub fn main() {
    common::main();
    println!("\n--\n");

    generic::main();
    println!("\n--\n");

    elision::main();
    println!("\n--\n");
}
