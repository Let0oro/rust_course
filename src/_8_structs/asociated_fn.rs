use crate::_8_::in_fn::TaylorSwiftSong;



impl TaylorSwiftSong {
  fn new (title: String, release_year: u32, duration_secs: u32) -> Self {
    Self { title, release_year, duration_secs }
  }
}

pub fn main () {
  println!("Asociated functions:\n");
  
  println!(" - Are functions that are attached to a type");

  println!(" - For example, we had use the String::from() asociated function, is not a method of String, is a functionf declare in the space of the String scope declaration, 'lives directly on the String function'");
  println!(" - Other example is String::new(), a constructor, which is a function that returns a new instance of a type");
  println!(" - For convenience, we will always create a ::new fn as a constructor of our structs (but in the methods implementation), to do this, we dont add 'self' as parameter, this is the way to create associates functions and the diference with impl struct methods");

  println!("\nThis is an example:   
  pub fn new (title: String, release_year: u32, duration_secs: u32) -> Self {{
    Self {{ title, release_year, duration_secs }}
  }}, 
  The used fn MUST be public to allow access in OTHER FILES, anywhere we are use methods or associated fn.");

  let fortnight = TaylorSwiftSong::new(
    String::from("Fortnight"), 
    2024, 
    228
  );

  fortnight.display_song_info();

  println!("We can define multiple implementation blocks, rust will merge all in the struct");

  println!("---\n");
}