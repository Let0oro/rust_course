pub fn main () {
  println!("Exercise:\n");
  
  let mut trip = start_trip();
  visit_philadelphia(&mut trip);
  trip.push_str(" and ");
  visit_new_york(&mut trip);
  trip.push_str(" and ");
  visit_boston(& mut trip);
  trip.push_str(".");

  show_itinerary(&trip);
}

fn start_trip () -> String {
  let str = String::from("The plan is...");
  str
}

fn visit_philadelphia (str: &mut String) -> () {
   str.push_str("Philadelphia")
}

fn visit_new_york (str: &mut String) -> () {
  str.push_str("New York");
}

fn visit_boston (str: &mut String) -> () {
  str.push_str("Boston");
}

fn show_itinerary (str: &String) -> () {
  assert_eq!("The plan is...Philadelphia and New York and Boston.", str);
  println!("Trip: {str}");
}