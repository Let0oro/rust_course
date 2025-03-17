

pub fn main() {
  println!("Strings\n");

  println!("\tTabulator (\\t)");
  println!("\nNew line (\\n)");
  println!("\rCarriage return (\\r)");
  println!("\0Null (\\0)");
  println!("Backslash (\\)");
  println!("Double quote (\")");

  let raw_string: &str = r"C:\Users\User\Desktop\file.txt";

  println!("Filepath (C:\\Users\\User\\Desktop\\file.txt) -> (C:\\\\Users\\\\User\\\\Desktop\\\\file.txt)...\n or a raw string (r\"C:\\Users\\User\\Desktop\\file.txt\") -> {}", raw_string);

  println!("\nString methods\n");
  
  let _string: &str = "Hello, world!";
  let _string: String = String::from("Hello, world!");

  let mut string: String = String::from("Hello, ");
  let world: &str = "world!";
  string.push_str(world);
  string.push('!');

  let string: String = String::from("Hello, ");
  let world: &str = "world!";
  let hello_world: String = format!("{}{}", string, world);

  println!("&str: {}", string);
  println!("String: {}", string);

  println!("Formatted String like in the println!: {}", hello_world);

  println!("String length: {}", string.len());
  println!("String is empty: {}", string.is_empty());

  println!("String contains 'world': {}", string.contains("world"));
  println!("&str formatted into a String: String::from(&str): {}", String::from(world));
  println!("String into a &str: &String[..]: {}", &string[..]);

  println!("String split by whitespace: {:?} (str > iterator > collection)", string.split(" ").collect::<Vec<&str>>());

  println!("String replace 'world' with 'there': {}", string.replace("world", "there"));

  println!("String capacity: {}", string.capacity());

  println!("Justified (:>number): {number:>10}", number="0");
  println!("Pad Start (:number>number): {number:1>5}", number="0");
  println!("Pad End (:number<number):   {number:1<5}", number="0");
  println!("Custom Pad (:number>variable$):{number:1>width$}", number="0", width=5);

  println!("---\n")
}