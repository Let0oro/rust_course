#[derive(Debug)]
struct Coffee {
  price: f64,
  name: String,
  is_hot: bool
}

#[derive(Debug)]
pub struct TaylorSwiftSong {
  pub title: String,
  pub release_year: u32,
  pub duration_secs: u32,
}

impl TaylorSwiftSong {

  // fn display_song_info(self: TaylorSwiftSong) { // Immutable value 
  pub fn display_song_info(self: &Self) {
    let years_passed = self.years_since_release();
    let sing_plur =  if years_passed != 1 {"s"} else {""};
    println!("\nTitle: {}", self.title);
    println!("Release Year: {}, so it was released {} year{} ago", self.release_year, &years_passed, sing_plur);
    println!("Duration: {} seconds\n", self.duration_secs);
  }

  // fn double_length(mut self: Self) { // Mutable value
  fn double_length(self: &mut Self) {
    self.duration_secs *= 2;
  }

  fn is_longer_than (&self, other: &Self) -> bool {
     self.duration_secs > other.duration_secs
  }

  fn years_since_release (&self) -> u32 {
    2025 - self.release_year
  }
}

pub fn main () {

  let name = String::from("Latte");

  let mut latte = make_coffee(name, 1.50, true);

    println!(
      "Latte: 
  name = {} (latte.name), 
  price = {} (latte.price), 
  is_hot = {} (latte.is_hot)",
      latte.name, latte.price, latte.is_hot
  );
  
  latte.name = String::from("Con leche");
  
  let caramel_macchiato = Coffee {
    name: String::from("Caramel Macchiato"),
    ..latte
  };
 
  println!(
      "Caramel Macchiato: 
  name = {} (caramel_macchiato.name), 
  price = {} (caramel_macchiato.price), 
  is_hot = {} (caramel_macchiato.is_hot)",
      caramel_macchiato.name, caramel_macchiato.price, caramel_macchiato.is_hot
  );

  println!("If i do some like this creatign a new Coffee instance -> ..mocha. The result wolud be some like this:
  name: mocha.name, ..mocha.
  The problem is that mocha.name is a String, in the heap memory, without the Copy trait, so the ownership is changing to the new instance.
  The solution is: 'name: mocha.name.clone(), ..mocha'");

  drink_coffee(&caramel_macchiato);

  println!("{}", caramel_macchiato.name);

  change_coffee(&mut latte, "Latte");

  println!("New latte name! -> {}", latte.name);

  println!("Deriving our structs for implement the Display o Debug trait on there adding '#[derive(Debug)]' at he first file line");

  println!("{:#?}", latte);

  println!(" - In Rust, the states or values are defined in the 'struct' (structure) curly brackets, and the methods are defined in the 'impl <struct_name>' (implementation) curly brackets");

  println!("\nFor example: impl TaylorSwiftSong {{
      fn display_song_info(self: Self /*unnecesary to repeat the struct name, Self is an alias*/) {{
      // Immutable struct value (self parameter takes ownership)
      }};
      fn display_song_info(self: Self /*unnecesary to repeat the struct name, Self is an alias*/) {{
      // Mutable struct value (self parameter takes ownership, have permissions to mutate)
      }};
      fn display_song_info(self: &Self /*unnecesary to repeat the struct name, Self is an alias*/) {{
      // Immutable reference to the struct instance (no ownership moved)
      }};
      fn display_song_info(self: &mut Self /*unnecesary to repeat the struct name, Self is an alias*/) {{
        // Mutable reference to the struct instance (no ownership moved, have permissions to mutate)
      }};
  }}\n"); 

  let mut blank_space = TaylorSwiftSong {
    title: String::from("Blank Space"),
    release_year: 2014,
    duration_secs: 231,
  };

  blank_space.display_song_info();
  blank_space.double_length();
  println!("duplicating length...");
  blank_space.display_song_info();

  let all_too_well: TaylorSwiftSong = TaylorSwiftSong { 
    title: String::from("All Too Well"), 
    release_year: 2012, 
    duration_secs: 327,
  };

  if blank_space.is_longer_than(&all_too_well) {
    println!("{} is longer than {}", blank_space.title, all_too_well.title);
  } else {
    println!("{} is longer than or equal to {}", all_too_well.title, blank_space.title);
  }

  all_too_well.display_song_info();

  println!("---\n")

}


fn make_coffee (name: String, price: f64, is_hot: bool) -> Coffee {
  Coffee { price, name, is_hot }
}

fn drink_coffee (coffee: &Coffee) {
  println!("Drinking my delicious {}", (*coffee).name) // this occurs automatically with the Display trait, the usual way is simply coffee.name
}

fn change_coffee (coffee: &mut Coffee, new: &str) {
  coffee.name = String::from(new);
}
