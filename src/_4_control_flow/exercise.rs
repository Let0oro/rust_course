/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
 
Refactor the function above to use the `match` statement
instead of if, else if, and else.
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

pub fn main () {
  println!("\nExercise: Control flow");

  println!("Testing color_to_number function:");
  println!("color_to_number(\"red\") = {} (expected: 1)", color_to_number("red"));


  println!("\nTesting factorial function:");
  println!("factorial(5) = {} (expected: 120)", factorial(5));

  println!("\nTesting factorial_recursive function:");
  println!("factorial_recursive(4) = {} (expected: 24)", factorial_recursive(4));

  println!("---\n");
}

fn color_to_number(color: &str) -> u32 {
  match color {
    "red" => 1,
    "green" => 2,
    "blue" => 3,
    _ => 0,
  }
}

fn factorial(n: u32) -> u32 {
  let mut result: u32 = 1;
  for i in 1..=n {
    result *= i;
  }
  result
}

fn factorial_recursive(n: u32) -> u32 {
  if n == 0 {
    1
  } else {
    n * factorial_recursive(n - 1)
  }
}