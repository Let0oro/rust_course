

pub fn main () {
  println!("Ownership with arrays and tuplas\n");
  
  println!("Arrays: ");
  
  println!(" - let arr = [...] (arr is the owner of the array), but each element into will have the ownership of the array, like parameters");
  println!("Examples: ");
  
  let registrations = [true, false, true];
  let first = registrations[0]; // copied
  println!("   arr: [bool, 3] > first = arr[0] -> Copy trait is implemented here and there is no problems");
  println!("{first} and {registrations:?}");
  
  let languages = [String::from("Rust"), String::from("Javascript")];
  let first = &languages[0];
  println!(" * If i do \" let first = languages[0]; \", The array would be the partial owner of the value, and the other owner would be the 'first' name ");
  println!("   arr: [String, 2] > first = arr[0] -> Copy trait is NOT implemented here and there is a problem, solution will be to clone or to reference this value");
  println!("{first} and {registrations:?}");
  
  
  println!("Tuplas:");
  
  let registrations = (true, false, true);
  let first = registrations.0; // copied
  println!("   tup: (bool, bool, bool) > first = tup.0 -> Copy trait is implemented here and there is no problems");
  println!("{first} and {registrations:?}");
  
  let languages = (String::from("Rust"), String::from("Javascript"));
  println!(" * If i do \" let first = languages.0; \", The tupla would be the partial owner of the value, and the other owner would be the 'first' name ");
  let first = &languages.0;
  println!("   tup: (String, String) > first = tup.0 -> Copy trait is NOT implemented here and there is a problem, solution will be to clone or to reference this value");
  println!("{first} and {registrations:?}");

  println!("---\n");
}