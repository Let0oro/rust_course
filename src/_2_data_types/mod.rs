mod integers;
mod sizes;
mod strings;
mod floats;
mod casting_types; 
use casting_types as casting;
mod chars;
mod arrays;
mod traits;
mod tuples;
mod ranges;
mod exercise;

pub fn main () {
  integers::main();
  sizes::main();
  strings::main();
  floats::main();
  casting::main();
  chars::main();
  arrays::main();
  traits::main();
  tuples::main();
  ranges::main();
  exercise::main();
}