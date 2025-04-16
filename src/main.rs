#![allow(unused_variables)] // debug reasons
#![allow(unreachable_code)] // debug reasons

// "use" sirve para concretar qué es lo que vas a usar de una importación, por ejemplo, de la siguiente usaremos "Select" y valdrá como el nombre completo
use dialoguer::Select;
use std::process::Command;

// "mod" sirve para importar otros archivos, estos últimos tienen que estar al mismo nivel que este directorio, en una carpeta llamada "mod" o en un archivo con este mismo nombre
mod _0_setup;
mod _1_variables_and_mutability;
mod _2_data_types;
mod _3_functions;
mod _4_control_flow;
mod _5_ownership;
mod _6_ref_borrow;
mod _7_slices;
mod _8_structs;
mod _9_enums;
mod _10_generics;
mod _11_opt_result_enums;
mod _12_vectors;
mod _13_warehouse;
mod _14_strings;
mod _15_hashmap;
mod _16_error_handling;
mod _17_traits;
mod _17_traits_2;
mod _17_traits_3;
mod _18_lifetimes;

use _0_setup::print::prints;

use _1_variables_and_mutability::{constants, guess_number, variables, exercise};

use _2_data_types as _2_; //_2_.../mod.rs

use _3_functions as _3_;

use _4_control_flow as _4_;

use self::{
    _5_ownership as _5_,  _6_ref_borrow as _6_,          _7_slices as _7_,    _8_structs as _8_,     _9_enums as _9_,
    _10_generics as _10_, _11_opt_result_enums as _11_,  _12_vectors as _12_, _13_warehouse as _13_, _14_strings as _14_,
    _15_hashmap as _15_,  _16_error_handling as _16_,    _17_traits as _17_, _17_traits_2 as _17_2, _17_traits_3 as _17_3,
    _18_lifetimes as _18_,
};


fn main() {
    // vector (o array de tamaño no fijo) para seleccionar en consola el temario en concreto que se quiera ver
    let options: Vec<&str> = vec![
        "Prints",
        "Variables and mutability",
        "Data types",
        "Functions",
        "Control flow",
        "Ownership",
        "Reference and Borrowing",
        "Slices",
        "Structs",
        "Enums",
        "Generics",
        "Option And Result Enums",
        "Vectors",
        "Project Structure",
        "Strings",
        "Hashmap",
        "Error handling",
        "Traits",
        "Traits 2",
        "Traits 3",
        "Lifetimes",
    ];

    clear_console();

    let selection: usize = select_an_option(&options, "Select an option"); // Referencia a options, no se usa el ownership de options
    // let selection: usize = select_an_option(options.clone(), "Select an option"); / arg options: Vec<&str> // Clona options, se usa el ownership de options y se hace una copia que no le afecta // Menos eficiente y más uso de memoria

    cases_option_selected(options, selection);
}

// función para limpiar la consola dependiendo de si el OS es windows o macOS
fn clear_console() {
    let _clear: std::process::ExitStatus = match cfg!(target_os = "windows") {
        true => Command::new("cmd").args(&["/C", "cls"]).status().unwrap(),
        false => Command::new("clear").status().unwrap(),
    };
}

// función para seleccionar una opción a partir de un array de strings con un título personalizado, el usuario podrá actuar gracias a .interact(), lo que devolverá el index escogido
fn select_an_option(options: &[&str], title: &str) -> usize {
    Select::new()
        .with_prompt(title)
        .default(0)
        .items(&options)
        .interact()
        .unwrap()
}

// función que sirve como condicional (como un switch en js), no devuelve nada sino que ejecuta el código adjuntado a cada opción
fn cases_option_selected(options: Vec<&str>, selection: usize) {
    match options[selection] {
        "Prints" => prints(),
        "Variables and mutability" => {
            variables::main();
            constants::main();
            exercise::main();
            guess_number::main();
        }
        "Data types" => _2_::main(),
        "Functions" => _3_::main(),
        "Control flow" => _4_::main(),
        "Ownership" => _5_::main(),
        "Reference and Borrowing" => _6_::main(),
        "Slices" => _7_::main(),
        "Structs" => _8_::main(),
        "Enums" => _9_::main(),
        "Generics" => _10_::main(),
        "Option And Result Enums" => _11_::main(),
        "Vectors" => _12_::main(),
        "Project Structure" => _13_::main(),
        "Strings" => _14_::main(),
        "Hashmap" => _15_::main(),
        "Error handling" => _16_::main(),
        "Traits" => _17_::main(),
        "Traits 2" => _17_2::main(),
        "Traits 3" => _17_3::main(),
        "Lifetimes" => _18_::main(),

        _ => println!("Please select a valid option"),
    }
}
