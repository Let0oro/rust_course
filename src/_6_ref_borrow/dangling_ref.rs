

pub fn main () {
  println!("Dangling reference");

  println!(" - Is a pointer to a memory address that has been deallocated or cleared.");
  println!(" - Is an unreliable references, means that is a reference to a value that is no longer present.");
  println!("\nFor example: I cant return a reference to a value that has been created into a external main function cause when this function ends, the value will be cleared (end of its scope)");
  println!("\"(text fn) fn create_city -> &String (city) > let ref = create_city()\" - The existence of the city value has been cleared.");
  println!("Solution: fn create_city -> String. Returning the original value, not a reference.");
  let city = create_city();
  println!("my city: {}", city);

  println!("---\n");
}

fn create_city () -> String  {
  let city = String::from("New York");
  city
}