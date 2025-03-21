mod dangling_ref;
mod exercise;
mod immut_ref;
mod mut_ref;
mod owner_arr_tuplas;

pub fn main() {
    println!("\n References and Borrowing");

    immut_ref::main();
    mut_ref::main();
    dangling_ref::main();
    owner_arr_tuplas::main();
    exercise::main();

    println!("---\n");
}
