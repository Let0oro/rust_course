#![allow(unused_variables)] // debug reasons
#![allow(unreachable_code)] // debug reasons

// "use" sirve para concretar qué es lo que vas a usar de una importación, por ejemplo, de la siguiente usaremos "Select" y valdrá como el nombre completo
use dialoguer::Select;
use std::process::Command;

// "mod" sirve para importar otros archivos, estos últimos tienen que estar al mismo nivel que este directorio, en una carpeta llamada "mod" o en un archivo con este mismo nombre
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

mod _7_slices;
use _7_slices as _7_;

mod _8_structs;
use _8_structs as _8_;

mod _9_enums;
use _9_enums as _9_;

mod _10_generics;
use _10_generics as _10_;

mod _11_opt_result_enums;
use _11_opt_result_enums as _11_;

mod _12_vectors;
use _12_vectors as _12_;

mod _13_warehouse;
use _13_warehouse as _13_;


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
        "Warehouse",
    ];

    clear_console();

    // return _8_::main(); // debug reasons

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
            _1_::exercise::main();
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
        "Warehouse" => _13_::main(),

        _ => println!("Please select a valid option"),
    }
}
