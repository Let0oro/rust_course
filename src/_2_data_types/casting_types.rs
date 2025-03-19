pub fn main() {
    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    println!(
        "miles_away: {}i32, {}i8, {}u8",
        miles_away, miles_away_i8, miles_away_u8
    );

    let miles_away: f64 = 100.3213123;
    let miles_away_f32: f32 = miles_away as f32;
    let miles_away_int: i32 = miles_away as i32;

    println!(
        "miles_away: {}f64, {}f32, {}i32",
        miles_away, miles_away_f32, miles_away_int
    );
}
