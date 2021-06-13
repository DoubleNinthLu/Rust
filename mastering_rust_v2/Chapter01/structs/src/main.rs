#[derive(Debug)]
struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += 120;
    println!("{:#?}", player);
}
fn main() {
    let name = "Alice".to_string();

    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };
    bump_player_score(player, 120);
}
