enum HockeyPostion {
    Wing,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPostion,
    goals_ytd: u8,
}

fn main() {
    let mut player = HockeyPlayer {
        name: String::from("Bryan Rust"),
        number: 17,
        position: HockeyPostion::Wing,
        goals_ytd: 7,
    };

    player.goals_ytd += 1;

    println!(
        "{} has scored {} goals this season {}",
        player.name, player.goals_ytd, player.number)
}
