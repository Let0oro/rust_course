mod file_management;
mod exercise;

//use std::char::REPLACEMENT_CHARACTER;
//use std::process;

pub fn main() {
    println!("\n---\n");

    println!("Error handling");

    println!("Is the process of dealing with potential errors from operations that can go wrong");

    println!("We can group errors in two different categories: recoverable and unrecoverable");
    println!(" - Recoverable: Are errors that we can define code to handle");
    println!(" - Recoverable: Are errors that cause the program to be unable to proceed");

    println!("When the program found an error in execution time, we call this situation as a panic, an example is the result of 'None.unwrap()'");

    println!("The backtrace is the list of files and functions that were running at the point that error occurred (to localize it), is the sequential list of events before the panic");

    println!("We can force a panic with the macro 'panic!(\"message\")'");

    println!("When a panic occurs, rust will remove all the data from heap and stack to safe our computer memory");

    println!("If we want to end the process of our program in the computer, we can use the exit namespace from the standard library 'std::process::exit(code: <i32>)'");

    println!("process::exit(0); -> No errors exists");
    println!("process::exit(1); -> (or more) Exists an error");

    println!("\n--\n");

    println!("The 'eprintln!' macro:");

    println!("Print a message like prinln! macro when we use the common 'cargo run' command BUT:");

    println!("If we use the command 'cargo run > example.txt', that points to 'example.txt' file or create it with all prints of our program, eprintln will keep the messages in terminal, but prinln messages will be typed in the file");

    println!("So this will be separate the prints in two different channels");

    eprintln!("Some error message");

    println!("\n--\n");

    file_management::main();

    println!("\n--\n");

    println!("Error type declarations:");

    println!("Depends on the module from arrive");
    println!("io::Error -> Input/Output operations");
    println!("fmt::Error -> Read or Write operations in console");
    println!("We can declare our Error types");

    println!("\n--\n");

    println!("The try operator ('?')");

    println!("RESULT ENUM:");
    println!("

    'let value = operation();

    if let Err(error) = value {{ return Err(error); }};'

    ---This is equal to:---

    'operation()?;'

    ");

    println!("

    'let mut file = match File::open(&input.trim()) {{
        Ok(file) => file,
        Err(error) => return Err(error),
    }};'

    ---This is equal to:---

    'let mut file = match File::open(&input.trim())?;'

    ");

    println!("* While the methods produce a Result enum, we can use this operator");

    println!("For example:

        'let mut file = match File::open(&input.trim()) {{
            Ok(file) => file,
            Err(error) => return Err(error),
        }};

        let mut file_contents = String::new();

        if let Err(error) = file.read_to_string(&mut file_contents) {{
            return Err(error);
        }};'

    ---This is equal to:---

        'let mut file_contents = String::new();
        File::open(&input.trim())?.read_to_string(&mut file_contents)?;'

    ");

    println!("\n--\n");

    println!("The 'read_to_string' associated function:");

    println!(" - Reads the entire contents of a file into a string.");
    println!(" - This is a convenience function for using File::open and read_to_string with fewer imports and without an intermediate variable.");
    println!(" - This function will return an error if path does not already exist.");

    println!("This '
        let mut file_contents = String::new();
        File::open(&input.trim())?.read_to_string(&mut file_contents)?;
    ' is equal to: 'let file_contents = fs::read_to_string(&input.trim())?;'");

    println!("\n--\n");

    println!("Try operator ('?') with OPTION ENUM:");

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];

    fn length_of_last_element (input: &mut Vec<&str>) -> Option<usize> {
        let last_element = input.pop()?;
        Some(last_element.len())
    }

    let last_animal = length_of_last_element(&mut animals);
    println!("{:?}", last_animal);


    println!("\n---\n");
}