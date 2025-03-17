
pub fn main() {

  println!("Functions exercises: \n");
  apply_to_jobs(35, "Rust Developer");

  is_even(8);
  is_even(9);

  println!("{:?}", alphabets("aardvark"));
  println!("{:?}", alphabets("zoology"));
  println!("{:?}", alphabets("zebra")); 
}

fn apply_to_jobs (number: i32, title: &str) -> () {
  println!("I'm applying to {number} {title} jobs");
}

fn is_even(int: i32) -> bool {
  return int % 2 == 0;
}

fn alphabets(text: &str) -> (bool, bool) {
  (text.contains("a"), text.contains("z"))
}