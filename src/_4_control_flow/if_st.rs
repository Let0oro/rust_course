

pub fn main() {
  let  age = 40;

  let cond_greets: &str = if age < 30 { "Hello, Young" } else if age < 15 { "Hello, Kid" } else { "Hello, Sir" };
  println!("{}", cond_greets);
}