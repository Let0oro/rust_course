
// Hours, minutes
struct ShortDuration (u32, u32);

// Years, months
#[allow(dead_code)]
struct LongDuration (u32, u32);

pub fn main () {
  println!("Tuple Structs:\n");
  
  println!(" - A Tuple Struct is a struct that assign each piece of data an order in line (index) rather than a name");

  let work_shift = ShortDuration(8, 0);
  println!("{} hours, {} minutes", work_shift.0, work_shift.1);
  
  
  let era = ShortDuration(5, 3);
  println!("{} years, {} months", era.0, era.1);

  println!("\n¿Why not to use a normal tuple instead?:");
  println!(" - Is not the same type of a tuple, not corresponding and is not allow to use a tuple in the same place.");
  println!(" - The name of the struct serves as an identifier of this type of data, so its less probably to do type errors.");
  println!("    For example: if I want the struct ShortDuration will be the waiter parameter of a function, this will avoid to use a simple tuple or another tuple struct, means in a better code.");
  



  println!("---\n");
}