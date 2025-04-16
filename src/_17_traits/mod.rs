/*!
Traits:

 - Is a contract hath describes functionality that a type should have
 - We use the word implement to describe when a type honors a trait's requirements.
 - A trait definition declares the method(s) that a type implementing that trait mush have:
   -> Method name
   -> Parameter with type
   -> Return value type

The Display and Debug traits require a type to define methods tor presenting itself as a string.
The Clone trait requires a type to define a clone method for creating a duplicate of itself


Trait Objects -> instance of a type that implements a particular trait whose methods will be accessed at runtime using a feature called dynamic dispatch

We need to have the trait in the same scope module we are using it:
a.add(b) needs the Add trait that implements the .add method, so we need to type "use std::ops::Add;" up on the file, even when we aren't using the literal trait name

Recommended refactor tree: (imagine our mod.rs is actually a main.rs binary crate)
    traits/
    |-main.rs (with "use traits::..." to access to the lib functionality)
    |-lib.rs (with "mod lodging" and "mod utils")
    |-utils.rs
    |-lodging/
         |-mod.rs
*/

use std::collections::HashMap;
use std::fmt::Display;
use _Number::{Single, Many};

/// This trait requires that every type implementing this will must include the methods and its types, we can define a default method like get_description, modify or let it be for the elements this trait implement
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();

    fn like_ref(self) -> &'static Self {&self}
}

trait Description {
    fn get_description(&self) -> String {
        String::from(" wonderful place to stay")
    }
}

/** We are asking Rust to do the default implementation of the Debug trait in our Hotel struct, we can define methods and call from it other trait methods \
 Trait bound to conditionally implement methods -> Implementing the Display trait as bound in the generic type of Hotel in the name property
*/
#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new()
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Description for Hotel<T> {}

/// We are implementing the Accommodation trait -> impl name_trait for name_struct
impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnb {
    fn new(name: &str) -> Self {
        Self { host: name.to_string(), guests: vec![] }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) -> () {}
}

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!("please, enjoy {}'s apartment", self.host)
    }
}

///We can call a function who accept a trait and, at this way, this accepts as parameter every element who has this trait implemented
fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) -> () {
    entity.book(guest, 1);
}

/**
Trait bound (the same as up code but without 'sintaxis sugar', more simple)

 - Requires that a generic type implement a specific trait, making a bound to the generic

*/
fn book_for_two_nights<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 2)
}

enum _Number<T> {
    Single(T),
    Many(Vec<T>),
}

/**Trait bound again
    - Use 'dyn' for make the type dynamic, not necessary equal in every case -> This is a trait object
    - To point multiple traits, we can 'sum' the trait like this: '(impl trait + trait)'
*/
fn mix_and_match<T>(places: _Number<&mut (dyn T)>, guest: &str)
where
    T: Accommodation + Description
{
    match places {
        Single(place) => book_for_one_night(place, guest),
        Many(places) => places.iter().for_each(|place: T| book_for_one_night(place, guest)),
    }
}

/**Where clauses
    - Simplify to declare specific types cleaner than in the place of the generics
*/
fn mix_and_match_where<T, U> (first: &mut T, second: &mut U, guest: &str)
    where
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

/// Traits as Function Return values -> keep watch the returned type is all "impl Accommodation + Description", both, no one separated of other
fn choose_best_place_to_stay () -> impl Accommodation + Description {
    Hotel::new("The Luxe");
}


pub fn main () {
    let mut deluxe = Hotel::new("The Luxe");
    let mut my_house = AirBnb::new("Peter");

    mix_and_match(Many(vec![&mut my_house, &mut deluxe]), "Sergio");
    let mut hotel_without_display = Hotel::new(vec!["Notdis Play"]);
    println!("{:?} not implements display method and not reach summarize method", hotel_without_display);
}