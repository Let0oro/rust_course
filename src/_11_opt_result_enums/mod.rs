mod opt_enum;
mod real_opt_enum;
mod result_enum;
mod real_result_enum;
mod while_let;
mod exercise;

pub fn main() {
    println!("\nOption and Result Enums:\n");

    println!(" - Are two of the more common enums predefined or native from Rust.\n");


    opt_enum::main();
    real_opt_enum::main();
    result_enum::main();
    real_result_enum::main();
    while_let::main();
    exercise::main();


    println!("---\n");
}