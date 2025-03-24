mod multiple_generics;
mod structs_generics;
mod turbofish_op;
mod enum_generics;
mod exercise;

#[derive(Debug)]
struct DeliSandwich {
    name: String,
    score: f64,
    repeat_future: bool,
}

pub fn main() {
    println!("Generics:\n");

    println!(" - A generic is a type argument, it's a placeholder for a future type.");

    println!("\nFor example: fn identity<T>(value: T) -> T {{ value }}");
    println!(" -> '<T>': The arbitrary name of our generic type declaration, necessary");
    println!(" -> '(value: T)': using the generic type as the waited for the argument");
    println!(" -> '-> T': using the generic type as th waited to be returned\n");

    println!("Other example: fn identity<HELLO>(value: HELLO) -> HELLO {{ value }}\n");

    println!("{}", identity(5));
    println!("{}", identity(true));
    println!("{}", identity(13.12));
    println!("{}", identity(String::from("Hello, World!")));
    println!("{}", identity(&String::from("Hello, World!")));
    println!("{}", identity("Hello, World!"));
    println!("{:?}", identity(("Hello", String::from("World"), 12,)));
    println!(
        "{:?}\n",
        identity(DeliSandwich {
            name: String::from("Hello"),
            score: 4.5,
            repeat_future: true
        })
    );

    println!(
        " - Rust will read all the invocations of the function with generics and will make a different function of each every possibility for us."
    );
    println!(
        " - So we can use the generics and keep safe cause Rust is saving us from any risks with the types"
    );
    println!(" - This process is called 'Monomorphization'\n");

    turbofish_op::main();
    multiple_generics::main();
    structs_generics::main();
    enum_generics::main();
    exercise::main();

    println!("---\n");
}

fn identity<T>(value: T) -> T {
    value
}
