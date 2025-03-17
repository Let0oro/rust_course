

pub fn main() {
    let season = "summer";
    let attitude = seasons_attitude(season);
    println!("{}: {}", season, attitude);

    let season = "fall";
    let attitude = seasons_attitude(season);
    println!("{}: {}", season, attitude);

    let season = "winter";
    let attitude = seasons_attitude(season);
    println!("{}: {}", season, attitude);

    let season = "spring";
    let attitude = seasons_attitude(season);
    println!("{}: {}", season, attitude);

    let season = "autumn";
    let attitude = seasons_attitude(season);
    println!("{}: {}", season, attitude);

    let season = "summer";
    let attitude = seasons_attitude_v2(season);
    println!("{}: {}", season, attitude);

    let season = "fall";
    let attitude = seasons_attitude_v2(season);
    println!("{}: {}", season, attitude);
}

fn seasons_attitude(season: &str) -> &str {
    match season {
        "summer" => "I love summer!",
        "fall" => "I like fall.",
        "winter" => "I don't like winter.",
        "spring" => "I like spring.",
        _ => "I don't know that season.",
    }
}


fn seasons_attitude_v2(season: &str) -> &str {
  match season {
    "summer" | "winter" => "Hate it!",
    "fall" | "spring" => "Love it!",
    _ => unreachable!(),
  }
}