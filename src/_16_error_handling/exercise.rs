use std::{fs, io};
use std::io::{stdin};

pub fn main () {
    match write_to_file() {
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => { eprintln!("There was an error: {error}"); std::process::exit(1); },
    }
}

fn write_to_file () -> io::Result<String> {
    let input_file = get_user_input("What file would you like to write to?")?;
    let input_write = get_user_input("What would you like to write to the file?")?;

    fs::write(&input_file, input_write)?;

    Ok(input_file)
}

fn get_user_input (question: &str) -> io::Result<String> {
    println!("{}", question);

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}