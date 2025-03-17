

pub fn main () {
  println!("Traits\n (Example with Display and Debug)\n");

  // The Display trait is used to format the output in a more user-friendly way
  // The Debug trait is used to format the output in a more programmer-friendly way

  let seasons: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];


  println!("{}", 5);
  println!("{}", 3.14);
  println!("{}", true);
  println!("{}", "Hello, world!");
  println!("{}", seasons[0]);
  println!("Debug trait (:?): {:?}", seasons); // Debug trait
  println!("Debug trait with pretty output (:#?): {:#?}", seasons); // Debug trait

  let format_method = format!("{:?}", seasons);

  println!("Debug trait from format macro (format!(\"...\")): {}", format_method); // Debug trait

   // Debug trait

   dbg!(seasons); // Debug trait

   print!("Debug macro (dbg!(2+2)): ");
   let debug_add = dbg!(2 + 2);

  println!("dbg! macro (Debug trait) to print and return debug dirty value: {}", debug_add); // Debug trait

  println!("---\n");
}