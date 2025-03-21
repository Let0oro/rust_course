mod basics;
mod deref_coercion;
mod exercise;
mod mut_arr_slices;

pub fn main() {
    println!("Slices\n");

    basics::main();
    deref_coercion::main();
    mut_arr_slices::main();
    exercise::main();

    println!("---\n")
}
