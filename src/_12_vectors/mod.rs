mod exercise;

pub fn main () {
    println!("Vectors:");

    println!("Is a data collector similar to an array (sequentially same-type data collector), must have the same length in all program duration");

    println!("Array: [&str; 3] = [\"Gosthbusters\", \"Honey, I shrunk the kids\", \"Spaceballs\"]");

    let rick_moranis_movies: [&str; 3] = ["Gosthbusters", "Honey, I shrunk the kids", "Spaceballs"];

    println!("A vector is like a 'flexible array', must have the same type and this data collection is sequential");

    println!("Vector: Vec<i32> = Vec::new(), or Vec::<i32>::new()");

    let pizza_diameters = Vec::<i32>::new();

    println!("{:?}", pizza_diameters);

    let pastas: Vec<&str> = Vec::new();

    let pastas = Vec::<&str>::new();
    println!("{:?}", pastas);

    println!("There is a special macro to create vector -> vec![] -> vec![1, 4, 5, 10]");

    println!("\n--");

    println!("\nAdding and removing elements:\n");

    let mut pizza_diameters = vec![8, 12, 16, 18];

    println!("To add an element to the end of the vector, we can use .push(elem)");
    pizza_diameters.push(20);
    pizza_diameters.push(22);

    println!("To add an element at an arbitrary position, we can use .insert(index, elem)");
    pizza_diameters.insert(0, 4);

    println!("To remove an element from the end of the vector, we can use .pop() // returns the last element in a Some Option, or a None Option");
    pizza_diameters.pop();

    println!("To remove an element from an arbitrary position, we can use .remove(index)");
    pizza_diameters.remove(1);


    println!("\n--\n");

    println!("Reading vector elements:\n");

    println!("The common way would be with some like this: vector[index]");
    let piece_of_pizza = pizza_diameters[1];
    let night_movie = rick_moranis_movies[1..3];

    println!("But we knows a better way, more safe: vector.get(index) -> Option");
    let piece_of_pizza = pizza_diameters.get(1);
    let night_movie = rick_moranis_movies.get(1..3);

    println!("\n--\n");

    println!("Ownership in vectors:\n");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    println!("Don't have the Copy trait, it will be in the heap");
    println!("Then, this will follow the same ownership seen in the course, like in the String specially");

    println!("\n--\n");

    println!("Writing vector elements:\n");

    pizza_toppings[1] = String::from("Olives");

    println!("{pizza_toppings:#?}");

    let target_topping = &mut pizza_toppings[2];

    target_topping.push_str(" and Meatballs");
    println!("{pizza_toppings:#?}");

    println!("\n--\n");

    println!("Vector capacity:\n");

    println!("is the maximum number of elements that the vector can contain");

    let mut seasons = Vec::<&str>::with_capacity(4);

    println!("Length (.len): {}. Capacity (.capacity): {}", seasons.len(), seasons.capacity());

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");

    println!("Length (.len): {}. Capacity (.capacity): {}", seasons.len(), seasons.capacity());

    println!("When we add more than the capacity, rust deallocated the current heap memory and allocate on other larger, this can slow down the program, wasting memory and resources.");

    seasons.push("Summer");

    println!("Length (.len): {}. Capacity (.capacity): {}", seasons.len(), seasons.capacity());

    println!("\n--\n");

    exercise::main();

    println!("---");
}