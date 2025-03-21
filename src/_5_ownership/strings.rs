pub fn main() {
    println!("\nStrings");

    println!("\nThe &str type (immutable):");
    println!("  - Also called a string slice, is the most primitive string type.");
    println!("  - Its lenght is fixed and its characters previously defined.");
    println!("  - It is usually seen in its borrowed form, &str. ");
    println!(
        "  - It is also the most difficult string type to work with because of its lack of ownership and mutability."
    );

    println!("\nThe String type (mutable):");
    println!("  - Which is a heap-allocated string, is the most common string type.");
    println!("  - It is also the most flexible string type.");
    println!("  - It is the type you should use when you need to own or modify a string.");

    println!("\n===Examples:===");

    let food: &str = "pasta";
    println!(" let food: &str = 'pasta'; // food: {food}");

    let text: String = String::new();
    println!(" let text: String = String::new(); // text: {text}");

    let text_food: String = String::from(food);
    println!(
        " let text_food: String = String::from(food); // text_food: {text_food} -> referencia a food"
    );
    println!(" let text_food: String = String::from(\"food\"); // text_food: \"food\"");
    dbg!(text_food);

    println!("\nMethod push_str (String) - Assigns text on the finish of a string:");
    let mut text: String = String::new();

    text.push_str("Hello");
    println!(" text.push_str('Hello'); // text: {text}");

    let mut name = String::from("Boris");

    println!(
        "Memory prepared for name (before pushing): {:?}",
        name.capacity()
    );
    println!("Length of name: {:?} (before)", name.len());

    name.push_str(" Johnson");
    println!("name.push_str(' Johnson'); // name: {name}");

    println!("Memory prepared for name (after): {:?}", name.capacity());
    println!("Length of name: {:?} (after)", name.len());

    println!("\nString vs. &String vs. &str:");

    println!("  - String: A dynamic piece of text stored on the heap at runtime.");
    println!("  - &String (\"ref string\": A reference to a heap-allocated string.");
    println!(
        "  - str: A hardcoded, read-only piece of text encoded in the binary, never will see in the code cause is already the codifier text."
    );
    println!("  - &str: A reference to the binary text in memory.");

    let ice_cream: &str = "Cookies and Cream";
    let text_slice: &str = &ice_cream;

    let text: String = String::from("Hello");
    let text_ref: &String = &text;

    println!(" text (w/ push): {text}// String");

    println!("---\n")
}
