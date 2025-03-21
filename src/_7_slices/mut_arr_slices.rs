pub fn main() {
    println!("\nMutable array slices:");

    println!(
        " - Rust avoid mutable slices of &str, cause this is referencing a memory managed binary data..."
    );
    println!("But Rust allow mutable slices of arrays:\n");

    let mut my_array: [i32; 5] = [10, 15, 20, 25, 30];
    let my_slice: &mut [i32] = &mut my_array[2..4];
    println!("1. let mut my_array: [i32; 5] = [10, 15, 20, 25, 30]; Declaramos un array mutable");
    println!(
        "2. let my_slice: &mut [i32] = &mut my_array; Declaramos una referencia mutable del array"
    );

    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;

    println!(
        "3. my_slice[0] = 100; Cambiamos el valor de un elemento dentro de la referencia mutable del array"
    );

    println!(
        "My slice: {:?}. The value has been modified in the changed reference mutable.",
        my_slice
    );
    println!(
        "My array: {:?}. The value has been modified in the original array mutable, cause we always have worked with the reference to a original slice of the original array.",
        my_array
    );

    println!("---\n");
}
