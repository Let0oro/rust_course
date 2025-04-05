

pub fn main() {
    println!("The While Let Declaration:\n");
    let mut sauces = vec!["Mayonnaise", "Ketchup", "Ranch", "Mustache"];

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce is: {}", sauce);
    }

    // This but infinitely:

    println!("Previously we saw the 'if let' declaration, but in this case 'if let Some(sauce) = sauces.pop() {{}}', that take a lot of values, as long as the array length, and its interesting to use this other declaration:");
    println!("'while let Some(sauce) = sauces.pop() {{}}'");
    println!("This creates a loop while the declaration give a supposed 'true' value, activating their following block");
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is: {}", sauce);
    }

    println!("---\n");
}