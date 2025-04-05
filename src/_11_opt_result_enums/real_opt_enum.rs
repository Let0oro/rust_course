
#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(&self) -> i32 {
        // Desestructuring? Other example = let Something::Case(number) = Something::Case(10) -> number = 10
        // Other way to see it -> self === MyOption::Some(32) (the value from method calling)
        // So if the upper is true, this would be like:
        // let MyOption::Some(value) = /*self*/MyOption::Some(32) -> let value = 32
        // let Self::Some(value) = /*self*/Self::Some(32) -> let value = 32
        let Self::Some(value) = self else {
            panic!("It's impossible to unwrap it!");
        };
        *value
    }

    fn unwrap_or(self, default: i32) -> i32 {
        let Self::Some(value) = self else {
          return default;
        };
        value
    }
}

enum BorisOption {
    Some(i32),
    None,
}

impl BorisOption {
    fn unwrap(self) -> i32 {
        match self {
            BorisOption::Some(value) => value,
            BorisOption::None => panic!("It's impossible to unwrap it!"),
        }
    }

    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            Self::Some(value) => value,
            Self::None => default
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MyOptionTwo<T> {
    Some(T),
    None,
}

pub fn main () {
    println!("Real Example of Option Enum:\n");

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    println!("A secure option to get an element from an array is using the method '.get()', that returns an Option Enum, that prevents the possibility of select an out-of-range index");


    let bass = musical_instruments.get(2); // Type -> Option<&String>

    println!("
    Declare bass: 'let bass = musical_instruments.get(2);'
    Bass value: {:?} -> Some(\"Bass\")
    Type: Option<&String> CLARIFICATION -> The returned value is not a String Reference nor a String, but is a enum Option storing the actual value, its necessary to unwrap the value to actually get it.
    * .unwrap() -> Returns the contained Some value, consuming the self value. and panic! if not return data (generates an error if the returned value is None)
    Bass unwrapped value: {} -> \"Bass\"
    ",
    bass,
    bass.unwrap()
    );

    let invalid_instrument = musical_instruments.get(100);
    println!("Declare bass: 'let bass = musical_instruments.get(2);'");
    println!("Invalid instrument: {invalid_instrument:?} -> None");
    println!("Type: 'Option<&String>'");
    println!("Invalid instrument unwrapped: {{:invalid_instrument.unwrap()}} -> 'thread 'main' panicked at src\\_11_opt_result_enums\\real_opt_enum.rs:30:68: called `Option::unwrap()` on a `None` value'");
    println!("There's no capacity for error");

    let experiment = musical_instruments.get(1..2);
    println!("{experiment:?}\n");

    println!("--");

    println!("\nUnwrap and Expect methods:\n");

    println!(" - Ways to get the actual value from Some Option:");
    println!("   -> '.unwrap()': Attempts to extract the associated data out of the Some variant. Returns the actual value or an error (panic!)");

    let valid_instrument = bass.unwrap();
    println!("      bass unwrapped: {valid_instrument} -> Type: &String (reference of a String)");
    println!("      BAD always assume that there some to extract");

    println!("   -> '.expect()': is identical than 'unwrap' but allows us to customize the error message");

    let valid_instrument = bass.expect("Unable to retrieve musical instrument");
    println!("      bass expected: {valid_instrument} -> Type: &String (reference of a String)");
    println!("      Also will stop the program if find None Option, but we will get the message");
    println!("      Example on None Option value (invalid_element.expect(\"Unable to retrieve musical instrument\")) -> 'thread 'main' panicked at src\\_11_opt_result_enums\\real_opt_enum.rs:51:47:
Unable to retrieve musical instrument
'");

    println!("--");

    println!("\nThe match keyword with Option Enum:\n");
    println!(" - This is a common and well formed language solution to the problem before");

    match bass {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    };

    println!("
    match bass {{
        Option::Some(instrument) => println!(\"Playing the {{instrument}}\"),
                     ^^^^^^^^^^ -> arbitrary name for this block, the value will be the bass value unwrapped / expected
        Option::None => println!(\"Singing with my voice\"),
    }};

    * Option::Some === Some, the prefix is not necessary*
    ");

    match invalid_instrument {
        Some(instrument) => println!("Playing the {instrument}"),
        None => println!("Singing with my voice"),
    };

    println!("
    match invalid_instrument {{
        Some(instrument) => println!(\"Playing the {{instrument}}\"),
        None => println!(\"Singing with my voice\"),
    }};

    * Obviously we can refactoring this functionality passing the argument 'instrument_option: Option<&String>' and using 'match instrument_option'
    ");

    println!("IMPORTANT: Rust implement the Copy Trait on Option Enum, avoiding the borrowing");


    println!("--");

    println!("\nReturning an Option Enum from a Function:\n");

    let availability = is_item_in_stock(true, false);

    match availability {
        // Some(value) => println!("Item is available: {value}"), // generic and returning case
        Some(true) => println!("Yes, the item is available"), // Specific case
        Some(false) => println!("No, the item is not in stock"), // Specific case
        None => println!("Your item doesn't exist in our system"),
    }

    println!("
    fn is_item_in_stock (item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {{
        if item_is_in_system && item_is_in_stock {{
            Option::Some(true)
        }} else if item_is_in_system {{
            Option::Some(false)
        }} else {{
            Option::None
        }}
    }}

    fn main () {{
    ...
        let availability = is_item_in_stock(true, false);

        match availability {{
            // Some(value) => println!(\"Item is available: {{value}}\"), // generic and returning case
            Some(true) => println!(\"Yes, the item is available\"), // Specific case
            Some(false) => println!(\"No, the item is not in stock\"), // Specific case
            None => println!(\"Your item doesn't exist in our system\"),
        }}
    ...
    }}
    ");


    println!("*** The Rust prelude is a collection of named constructs that are available automatically in every program");
    println!("***  - For example, the prelude includes 'Option', but also it's varieties 'Some' and 'None'");
    println!("***  - 'Are all names we can use at 'top-level' (without importing libraries or specific features)");
    println!("***  - 'Are automatically available for coding within'\n");

    println!("--");

    println!("\nThe unwrap_or method:\n");

    println!(" - Similar to the unwrap method but offers an if-not value in None cases");

    println!("bass: {}", bass.unwrap());
    let default_value = String::from("None is selected");
    println!("invalid instrument {} -> let default_value = String::from(\"None is selected\");", invalid_instrument.unwrap_or(&default_value));


    println!("\n--\n");

    println!("Building Option from scratch");
    println!(" - We will build a simulation of the Option Enum called 'MyOption' for learn it better");


    println!("My attempt:

    impl MyOption {{
        fn unwrap(&self) -> i32 {{
            // Desestructuring? Other example = let Something::Case(number) = Something::Case(10) -> number = 10
            // Other way to see it -> self === MyOption::Some(32) (the value from method calling)
            // So if the upper is true, this would be like:
            // let MyOption::Some(value) = /*self*/MyOption::Some(32) -> let value = 32
            // let Self::Some(value) = /*self*/Self::Some(32) -> let value = 32

            let Self::Some(value) = self else {{
                panic!(\"It's impossible to unwrap it!\");
            }};
            *value
        }}

        fn unwrap_or(self, default: i32) -> i32 {{
            let Self::Some(value) = self else {{
              return default;
            }};
            value
        }}

    }}
    ");

    println!("Boris(teacher):

    impl BorisOption {{
        fn unwrap(self) -> i32 {{
            match self {{
                BorisOption::Some(value) => value,
                BorisOption::None => panic!(\"It's impossible to unwrap it!\"),
            }}
        }}

        fn unwrap_or(self, default: i32) -> i32 {{
            match self {{
                Self::Some(value) => value,
                Self::None => default
            }}
        }}

    }}

    ");

    let example = MyOption::Some(32).unwrap();
    println!("{}", example);




    println!("---\n");
}

fn is_item_in_stock (item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}