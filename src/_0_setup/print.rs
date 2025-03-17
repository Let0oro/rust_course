pub fn prints() {
    println!("Hello, world!\n");

    println!("This is a macro, the other one code that i can use (some like node.console.log)\n");

    eprintln!("This is an error message\n");

    println!(
        "I can use the 'rustfmt' command to format the code in a more readable way, and 'cargo fmt' to format all the files in my project\n"
    );

    println!(
        "I can make an executable program of this FILE with the command: 'rustc main.rs' and then execute it with the command: './main'\n"
    );

    println!(
        "I can make an executable program of this PROJECT with the command: 'cargo build' and then execute it with the command: './[project_name]' in our ./target/(debug | release) folder\n"
    );
    println!("- cargo build may be both two ways: 'DEBUG' or 'RELEASE'\n");

    println!(
        "- 'DEBUG' (cargo b / build): will make a fast and no optimiced compilation, a good way to find and fix errors, the result is a exe more large\n"
    );

    println!(
        "- 'RELEASE' (cargo b / build --release): takes more time in compliling but will make a better performance optimiced compilation and the result is the 'final version' of our program, smaller and more depured\n"
    );

    println!("In order to clean all the executables we had make, we can use 'cargo clean' \n");

    println!("Cargo build + run the program = 'cargo r / run'\n");

    println!("Cargo build + run the program in debug mode = 'cargo run --debug'\n");
    println!("Cargo build + run the program in release mode = 'cargo run --release'\n");

    println!(
        "Run the program in debug mode + show ALL the output and proccesses and memorial = 'cargo run --debug --verbose'\n"
    );
    println!(
        "Run the program in release mode + show ONLY the output = 'cargo run --release --quiet'\n"
    );

    println!("To check the violeations of the compiler, we can use 'cargo check'\n");

    print!("Hello, i'm an inline print... ");
    print!("in the same line that the last print\n");

    // This is a comment
    /* This is another comment */

    println!("\n");
}
