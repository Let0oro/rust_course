use std::collections::HashMap;

pub fn main () {

    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("{:?}", sauces_to_meals.remove("Mayonnaise").unwrap());
    println!("{:?}", sauces_to_meals.get("Mustard").unwrap());

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);

}