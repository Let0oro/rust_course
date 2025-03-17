
pub fn main() {
  let c: char = 'z';
  let z: char = 'ℤ';

  println!("c: {}, z: {}", c, z);

  let heart_eyed_cat: char = '😻';

  println!("heart_eyed_cat: {}", heart_eyed_cat);

  let face: char = '\u{1F600}';
  println!("face: {}", face);

  println!("size of c: {}", std::mem::size_of_val(&c));
  println!("size of face: {}", std::mem::size_of_val(&face));

  println!("Is alphabetical? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_alphabetic(), z.is_alphabetic(), heart_eyed_cat.is_alphabetic(), face.is_alphabetic());

  println!("Is numeric? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_numeric(), z.is_numeric(), heart_eyed_cat.is_numeric(), face.is_numeric());

  println!("Is alphanumeric? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_alphanumeric(), z.is_alphanumeric(), heart_eyed_cat.is_alphanumeric(), face.is_alphanumeric());

  println!("Is whitespace? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_whitespace(), z.is_whitespace(), heart_eyed_cat.is_whitespace(), face.is_whitespace());

  println!("Is uppercase? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_uppercase(), z.is_uppercase(), heart_eyed_cat.is_uppercase(), face.is_uppercase());

  println!("Is lowercase? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_lowercase(), z.is_lowercase(), heart_eyed_cat.is_lowercase(), face.is_lowercase());

  println!("Is control? c: {}, z: {}, heart_eyed_cat: {}, face: {}", c.is_control(), z.is_control(), heart_eyed_cat.is_control(), face.is_control());

}