pub fn main() {
    square(5);
    square_same(5);

    println!(
        "With return === without return but last expression? [{}]",
        square(5) == square_same(5)
    );
    assert_eq!(square(5), square_same(5));

    void();

    let result: i32 = block_body();
    println!("The result of the block body function is: {:?}", result);
}

//Explicit return values

fn square(number: i32) -> i32 {
    println!("Any last expression in a function is the returned value");
    number * number
}

// Implicit return values

fn square_same(number: i32) -> i32 {
    return number * number;
}

// Unit or 'void' functions ()
fn void() -> () {
    println!("This function returns nothing '-> ()', an empty tupla");
}

// Block body functions
fn block_body() -> i32 {
    let _false_number = 5;
    let result = {
        let number = 3;
        number + 1
    };
    println!("The result is: {}", result);
    result
}
