const TOUCHDOWN_POINTS: i32 = 6;

pub fn main() {
    let season: &str = "fall";

    let mut points_scored: i32 = 28;

    print!("{} points for Ravenclaw!", points_scored);

    points_scored = 35;

    #[allow(unused_variables)]
    let event_time: &str = "06:00";
    #[warn(unused_variables)]
    let event_time: i32 = 6;

    println!(
        "The past {season} our team scored {} points, which is {points} points per touchdown. The event took place at {1}.",
        points_scored,
        event_time,
        points = TOUCHDOWN_POINTS
    );

    let _favorite_beverage: &str = "coffee";

    #[allow(unused_variables)]
    let other_fav_beverage: &str = "tea";
}
