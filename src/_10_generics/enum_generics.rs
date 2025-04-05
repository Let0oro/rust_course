
#[allow(dead_code)]
#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

pub fn main() {
    println!("Enum Generics:\n");

    println!("Declaration:

    enum Cheesesteak<T> {{
        Plain,
        Topping(T),
    }}
    ");

    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);
    println!("{:?} (&str), {:?} (String), {:?} (&String)", mushroom, onions, bacon);

    println!("If we want to declare a variable with Plain, an error spams caused for:
     - 'type annotations needed for `Cheesesteak<_>`'
     - 'type must be known at this point'
     -> The solution will be annotate to the type an specific type for the enum, let me example this:
        · 'let plain: Cheesesteak<&str> = Cheesesteak::Plain;'
                            ^^^^^^ -> any type we want, it is important for value mutations and Rust prevent it

     -> Error example:
        · let mut plain: Cheesesteak<&str> = Cheesesteak::Plain;
          plain = Cheesesteak::Topping(34); -> 'expected `&str`, but found `i32`'.
        Solution: 'let mut plain: Cheesesteak<i32> = Cheesesteak::Plain;' OR plain = Cheesesteak::Topping(\"34\");
     ");

    let mut plain: Cheesesteak<i32> = Cheesesteak::Plain;
    plain = Cheesesteak::Topping(34);





    println!("---\n");
}