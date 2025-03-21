pub fn main() {
    println!("\nExercise:");

    let is_concert: bool = true;
    let is_event = is_concert;

    println!(
        "No pass ownership in boolean, two values: original and copy, two names, {is_concert}, {is_event}"
    );

    let sushi = "Salmon";
    let dinner = sushi;

    println!(
        "No pass ownership in &str (reference), two values: original and copy, two names, {sushi}, {dinner}"
    );

    let mut sushi_heap = String::from(sushi);
    let dinner_heap = &sushi_heap;

    println!(
        "Pass ownership in String (heap and mutable), one value, two names, {sushi_heap}, {dinner_heap}"
    );

    eat_meal(&mut sushi_heap);

    println!("[sushi] Salmon (&str) > [dinner] (&str) > [sushi_heap] (String) > [dinner_heap] (&String) > 
  [eat_meal param] (String)");

    println!(
        "We can access the string after eat_meal cause eat meal manage a global reference we can have access: {:?}",
        sushi_heap
    )
}

fn eat_meal(meal: &mut String) {
    meal.clear();
}
