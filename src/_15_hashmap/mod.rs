mod exercise;

use std::collections::{HashMap, HashSet};

pub fn main () {
    println!("\n---\n");

    println!("Hashmaps:");

    println!("Is a collection type that consists of key-value pairs");

    println!("Create:");
    println!("We can create a empty hashmap or a hasmap from an array or vector of tuples");

    let mut hm_ex = HashMap::new();
    let feature_isa = String::from("Rizos");
    let isa = String::from("Rizos");
    let mut hm_from_vec = HashMap::from([("Manza", "Peludo"), ("Sergio", "Rubio"), (&feature_isa, &isa)]);

    println!("Add elements:");

    hm_ex.insert("Manza", "Peludo");

    println!("Get elements:");
    hm_from_vec["Manza"]; // unsafe -> value or panic (error)
    hm_from_vec.get("Manza"); // safe -> Option enum

    println!("Remove elements:");

    hm_from_vec.remove("Manza"); // returns the value in Option enum
    hm_from_vec.remove_entry("Manza"); // returns the key and the value in Option enum

    println!("Good practices");
    println!("First: .entry ensures that the value exist or not, returning it, later we can modify or aggregate a value");

    hm_ex.entry("Manza").and_modify("David");

    hm_ex.entry("Samu").or_insert("Elvis");

    println!("\n--\n");

    println!("HashSet:");

    println!("Is a collection type that stores unique values");

    println!("Has the same methods than Hashmap but .entry");

    let mut set_ex = HashSet::from("Molly");

    set_ex.insert("David");
    set_ex.insert("Manza");

    set_ex.contains("David");

    set_ex.get("David");


    let set_friends = HashSet::from(["Isa", "Sergio", "Manza", "Samu", "Nico"]);

    println!("Union: Returns a new set with all elements:");
    println!("{:?}", set_ex.union(&set_friends)); // ["Isa", "Sergio", "Manza", "Samu", "Nico", "David", "Molly"]
    println!("{:?}", set_friends.union(&set_ex)); // ["Isa", "Sergio", "Manza", "Samu", "Nico", "David", "Molly"]

    println!("Difference: Returns a new set with the different element from first set:");
    println!("{:?}", set_ex.difference(&set_friends)); // ["David", "Molly"]
    println!("{:?}", set_friends.difference(&set_ex)); // ["Isa", "Sergio", "Samu", "Nico"]

    println!("Symetric_difference: Returns a new set with unique different elements of two sets:");
    println!("{:?}", set_ex.union(&set_friends)); // ["Isa", "Sergio", "Samu", "Nico", "David", "Molly"]
    println!("{:?}", set_friends.union(&set_ex)); // ["Isa", "Sergio", "Samu", "Nico", "David", "Molly"]

    println!("Is_disjoint: Returns true if the sets NOT have at least one same value, in other case, returns true:");
    println!("{:?}", set_ex.union(&set_friends)); // false
    println!("{:?}", set_friends.union(&set_ex)); // false

    println!("Is_subset: Returns true if the first set is a part of the second set:");
    println!("{:?}", set_ex.is_subset(&set_friends)); // false

    println!("Is_subset: Returns true if the second set is a part of the first set:");
    println!("{:?}", set_ex.is_superset(&set_friends)); // false

    println!("\n---\n");
}