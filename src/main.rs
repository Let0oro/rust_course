#![allow(unused_variables)] // debug reasons
#![allow(unreachable_code)] // debug reasons

use dialoguer::Select;
use std::process::Command;

mod _0_setup;
use _0_setup::print::prints;

mod _1_variables_and_mutability;
use _1_::constants;
use _1_::guess_number;
use _1_::variables;
use _1_variables_and_mutability as _1_; // _1_.../main.rs

mod _2_data_types;
use _2_data_types as _2_; //_2_.../mod.rs

mod _3_functions;
use _3_functions as _3_;

mod _4_control_flow;
use _4_control_flow as _4_;

mod _5_ownership;
use _5_ownership as _5_;

mod _6_ref_borrow;
use _6_ref_borrow as _6_;

fn main() {
    let options: Vec<&str> = vec![
        "Prints",
        "Variables and mutability",
        "Data types",
        "Functions",
        "Control flow",
        "Ownership",
        "Reference and Borrowing",
    ];

    clear_console();

    return  _6_::main(); // debug reasons

    let selection: usize = select_an_option(&options, "Select an option"); // Referencia a options, no se usa el ownership de options
    // let selection: usize = select_an_option(options.clone(), "Select an option"); / arg options: Vec<&str> // Clona options, se usa el ownership de options y se hace una copia que no le afecta // Menos eficiente y más uso de memoria

    cases_option_selected(options, selection);
}

fn clear_console() {
    let _clear: std::process::ExitStatus = match cfg!(target_os = "windows") {
        true => Command::new("cmd").args(&["/C", "cls"]).status().unwrap(),
        false => Command::new("clear").status().unwrap(),
    };
}

fn select_an_option(options: &[&str], title: &str) -> usize {
    Select::new()
        .with_prompt(title)
        .default(0)
        .items(&options)
        .interact()
        .unwrap()
}

fn cases_option_selected(options: Vec<&str>, selection: usize) {
    match options[selection] {
        "Prints" => prints(),
        "Variables and mutability" => {
            variables::main();
            constants::main();
            _1_::exercise::main();
            guess_number::main();
        }
        "Data types" => _2_::main(),
        "Functions" => _3_::main(),
        "Control flow" => _4_::main(),
        "Ownership" => _5_::main(),
        "Reference and Borrowing" => _6_::main(),

        _ => println!("Please select a valid option"),
    }
}
