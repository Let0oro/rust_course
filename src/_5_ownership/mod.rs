mod mem;
mod strings;
mod copy;
mod mutability;
mod returned;
mod exercise;

pub fn main() {
    println!("\nOwnership");

    println!(
        "Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector."
    );
    println!(
        "All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory."
    );
    println!(
        "Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time."
    );
    println!(
        "None of the ownership features slow down your program while it’s running."
    );

    mem::main();
    
    strings::main();

    copy::main();

    mutability::main();

    returned::main();

    exercise::main();

    println!("---\n")
}
