struct Game;
struct Enermy;
struct Hero;

trait GameEntity {
    fn init(&self) {
        println!("im entity");
    }
}

impl GameEntity for Enermy {
    fn init(&self) {
        println!("im Enermy");
    }
}
impl GameEntity for Hero {
    fn init(&self) {
        println!("im Hero");
    }
}

impl Game {
    fn load<T: GameEntity>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enermy);
    game.load(Hero);
}
