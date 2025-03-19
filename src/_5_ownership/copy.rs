

pub fn main () {
    println!("\nCopy");
    println!("  - Copy is a trait that allows a type to be copied by simply assigning it to another variable, allowing his future uses without cloning or referencing.");
    println!("  - Copy is only implemented for a few types, such as integers, booleans, REFERENCES, and characters.");
    println!("  - Copy is not implemented for types that require allocation, such as String.");
    println!("  - Copy is also not implemented for types that have a Drop trait, such as Vec<T>.");
    
    let int: i32 = 5;
    let int_copy: i32 = int;
    
    println!("  - int: {int}");
    println!("  - int_copy: {int_copy} (whitout clone)");
    
    let boolean: bool = true;
    let boolean_copy: bool = boolean;
    
    println!("  - boolean: {boolean}");
    println!("  - boolean_copy: {boolean_copy} (whitout clone)");
    
    let character: char = 'c';
    let character_copy: char = character;
    
    println!("  - character: {character}");
    println!("  - character_copy: {character_copy} (whitout clone)");
    
    let string: String = String::from("Hello");
    let string_copy: String = string.clone();
    
    println!("  - string: {string}");
    println!("  - string_copy: {string_copy} change of ownership, needs to be copied with clone()");
    
    let vec_or_mut_arr: Vec<i32> = vec![1, 2, 3];
    let vec_or_mut_arr_copy: Vec<i32> = vec_or_mut_arr.clone();

    println!("  - vec_or_mut_arr: {vec_or_mut_arr:#?}");
    println!("  - vec_or_mut_arr_copy: {vec_or_mut_arr_copy:#?} change of ownership, needs to be copied with clone()");

    let txt: &str = "Hello";
    let txt_copy: &str = txt;

    println!(" - txt: {txt}");
    println!(" - txt_copy: {txt_copy} (whitout clone cause is a copy of a reference)");

    
}