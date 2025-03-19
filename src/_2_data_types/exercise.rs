pub fn main() {
    let number = 1_337;

    let number_i16 = number as i16;

    let number_f32 = 12.345334323;
    println!("{:.3}", number_f32);

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let _is_acceptable_coffee = with_milk || with_sugar;

    let arr_num: [i8; 4] = [10, 15, 3, 89];

    let full_tuple = (number_i16, number_f32, is_my_type_of_coffee, arr_num);

    dbg!(full_tuple);
    println!("{:#?}", full_tuple);
}
