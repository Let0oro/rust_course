//! Functional programming trats a function like any other values in a program
//! 
//! A clousure is a function without a name. It is sometimes called an anonymous 
//! function or lambda.
//! "let fun = |arg?: i32| -> { ... }" 
//! "let fun = |arg?| ..." 
//! 
//! Will take ownership like a normal function (so a clousure with a heap value 
//! returning or consuming in, give a FnOnce and with a pipe value are the others)
//! 
//! Trait hierarchy:
//! 
//! **FnOnce** -> Clousure captures values by **move**. (will be invoked once):
//! "let name = String::from("Manza"); let read_only = || name;"
//!   /\
//!   |
//! **FnMut** -> Clousure captures values by **mutable reference**. 
//! (will be invoked multiple times): "let mut num = vec![1, 2, 3];  
//! let mut add_num = || num.push(100);"
//!  /\
//!  |
//! **Fn** -> Clousure captures values by **immutable reference** 
//! (read-only) or doesn't capture. (will be invoked multiple times):
//! "let mult_by = |val| val * 4", "let print_num = || println!("{100}");"
//! 
//! 
//! "move" keyword will move the ownership of a value to the clousure:
//! "let name = String::from("ALice");
//! let print_str = move || {
//!   println!("{name}")
//! };"
//! 
//! And the clousures affected by this could be called multiple times, like the owner of the value
//! 

pub fn main() {}
