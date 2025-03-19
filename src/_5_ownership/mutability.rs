pub fn main () {

  println!("\nMutability");

  let burger = String::from("Burger");
  add_fries(burger); // let meal  = burger

  let mut current_meal = String::new();
  println!("{:?}", &current_meal);
  add_flour(&mut current_meal);
  add_sugar(&mut current_meal);
  add_salt(&mut current_meal);

  println!("final meal: {}", current_meal);

  println!("---\n")
}

fn add_fries (mut meal: String) -> () {
  meal.push_str(" with fries");
  println!("Burger: {}", meal);
}

fn add_flour (meal: &mut String) {
  explication("add_flour");
  meal.push_str(", add flour");
}

fn add_sugar (meal: &mut String) {
  explication("add_sugar");
  meal.push_str(", add sugar");
}

fn add_salt (meal: &mut String) {
  explication("add_salt");
  meal.push_str(", add salt");
}

fn explication (name_fn: &str) {
  println!("\nFn {name_fn}:");
  println!("  ...PASO como ARGUMENTO una referencia mutable del valor mutable para mo perder el ownership");
  println!("  ...RECIBO como ARGUMENTO una referencia mutable del valor mutable (lo mismo que paso)");
  println!("  ...no necesito devolverlo, el scope de la memoria es global como RETORNO una referencia mutable del valor mutable (lo mismo que paso y recibo)\n");
}