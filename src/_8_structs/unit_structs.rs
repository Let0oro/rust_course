#[derive(Debug)]
struct Empty;

pub fn main () {
  println!("Unit Structs:\n");
 
  let my_empty_struct = Empty;

  println!("Represented as 'struct <namespace>;', and used as 'let my_empty_struct = Empty' -> {:#?}", my_empty_struct);

  println!(" * ALL STRUCT TYPES MAY HAVE A IMPL FUNCTIONALITY, EVEN WITH NO DATA TO WORK WITH (struct)");
  
  println!("---\n");
}