

pub fn main() {
    println!("Real Example of Enum Result:\n");

    let text = "50";
    let error_text = "Alabama";

    let parsed_text = text.parse::<i32>(); // Type > Result<i32, ParseIntError>

    println!("let parsed_text = text.parse::<i32>(); // Type > Result<i32, ParseIntError>");
    println!("parsed_text (\"50\") = {:?} -> Ok(50)", parsed_text);

    let parsed_error_text = error_text.parse::<i32>();
    println!("parsed_error_text (\"Alabama\") = {:?} -> Err(ParseIntError {{kind: InvalidDigit}})", parsed_error_text);


    println!("--");

    println!("Returning a Result from a function:");


    let result = divide(10.0, 2.0);
    println!("Result from Divide(10/2) is {:?}", result);
    let result = divide(10.0, 0.0);
    println!("Result from Divide(10/0) is {:?}", result);

    println!("
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {{
        if denominator == 0.0 {{ Err(\"Cannot divide by 0\".to_string()) }} else {{ Ok(numerator / denominator) }}
    }}

    match result {{
        Ok(calculation) => println!(\"Result: {{}}\", calculation),
        Err(message) => println!(\"Error: {{}}\", message),
    }};
    ");

    match result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message),
    };


    println!("--\n");

    println!("Result methods:\n");

    println!(" - The 'unwrap', 'unwrap_or' and 'expect' are available also for the Result Enum");
    println!(" - 'is_ok' ( -> bool), returns 'true' if Ok, 'false' if Err.");
    println!(" - 'is_err' ( -> bool), the opposite way, 'true' if Err, 'false' if Ok.");
    println!(" * Some interesting advices for the use of 'unwrap' method with Result:");
    println!("   *-> Take care about the trait of the stored value, if bring or not the Copy Trait (if not, we will move the ownership and borrow this in each unwrap/expect invocation)");
    println!("   *Example:

    fn operation(great_success: bool) -> Result<String, String> {{
        if great_success {{Ok(\"Success\")}} else {{Err(\"Error\")}}
    }}

    let my_result: Result<String, String> = operation(true);

    let content: String = match my_result {{ // ERROR: moving the ownership
        Ok(message) => message,
        Err(error) => error,
    }}

    println!(\"{{}}\", my_result.unwrap()); // ERROR: moving the ownership

    ---Solution 1:
    ...let content = match &my_result...

    ---Solution 2:
    fn operation(...) -> Result<&'static str, &'static str> {{...}}
                                 ^^^^^^^ -> make the value not dead after the block ended or make it impossible to move out its original owner?
    ^-> cause: '... let content: &str' = match ...'
    ");




    println!("---\n");
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { Err("Cannot divide by 0".to_string()) } else { Ok(numerator / denominator) }
}