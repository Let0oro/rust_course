/*

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.
 
Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/
#[derive(Debug)]
struct Flight {
  origin: String,
  destination: String,
  price: f64,
  passengers: u32
}

impl Flight {
  fn new (origin: String, destination: String, price: f64, passengers: u32) -> Self {
    Self { origin, destination, price, passengers }
  }

  fn change_destination (&mut self, new_destination: String) -> &mut Self {
    self.destination = new_destination;
    self
  }

  fn increase_price (&mut self) -> &mut Self {
    self.price *= 1.20;
    self
  }

  fn itinerary (&mut self) -> &mut Self {
    println!("({} -> {}) [{} passengers]", self.origin, self.destination, self.passengers);
    self
  }

  fn display_info (&mut self) -> &mut Self {
    println!("{:#?}", self);
    self
  }

}

pub fn main () {

  let mut friends_ship = Flight::new(String::from("Madrid"), String::from("Valladolid"), 40.0, 2);

  friends_ship
    .display_info()
    .change_destination(String::from("Alicante"))
    .itinerary()
    .display_info();

  let mut savage_ship = Flight {origin: String::from("Milan"), destination: String::from("London"), ..friends_ship};

  savage_ship
    .display_info()
    .increase_price()
    .display_info();
}