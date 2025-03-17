// Variables and Mutability
// #![allow(unused_variables)] // This will not throw an error in this file

pub fn main() {
    println!("Variables and Mutability\n");

    let apples: i32 = 50 + 20;
    let mut bananas: i32 = 2;
    println!("apples: {}", apples);
    println!("bananas: {}", bananas);

    let _not_used_variable: i32 = 10; // This will not throw an error

    // bananas = i32::pow(bananas, 30);
    bananas = bananas.pow(30);
    println!("more bananas: {}", bananas);

    let fruits: i32 = apples + bananas;
    println!(
        "fruits: {0}. Apples = {1}, bananas = {bananas:<5}, apples + apples? = {2}",
        fruits,
        apples,
        apples + apples,
        bananas = bananas
    );

    let _unmutable_variable: i32;

    _unmutable_variable = 10;

    // _unmutable_variable = 20; // This will throw an error

    let grams_of_protein: &str = "100.345";
    let grams_of_protein: f64 = grams_of_protein.parse::<f64>().expect("Not a number");

    assert_eq!(
        Ok::<f64, f32>(100.345),
        Ok(grams_of_protein),
        "we are testing equal types of {} and {}",
        100.345,
        grams_of_protein
    );
    println!("grams_of_protein: {}", grams_of_protein);

    #[allow(unused_variables)]
    let unadverticed_variable: i32 = 10;
    #[warn(unused_variables)]
    let _adverticed_variable: i32 = 10;

    // println!("Hey, it`s me, the tax constant: {TAX_RATE}\n");

    println!("\n");
}
