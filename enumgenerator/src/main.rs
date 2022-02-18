enum HockeyPosition {
    Center,
    LeftWing,
    RightWing,
    Defense,
    Goalie,
}

fn next_player(_position: HockeyPosition) {

}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8)
}

fn main() {
    let position = HockeyPosition::Defense;
    next_player(position);
}

