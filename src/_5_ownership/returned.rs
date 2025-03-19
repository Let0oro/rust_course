

pub fn main () {

  let cake: String = bake_cake();
  println!("Cake: {}", cake);
}

fn bake_cake() -> String {
  String::from("Chocolate mousse")
}