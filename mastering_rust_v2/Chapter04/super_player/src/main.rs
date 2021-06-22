mod media;
use media::Playable;

struct Audio(String);

struct Video(String);

impl Audio {
    fn new(a: &str) -> Self {
        Self(String::from(a))
    }
}

impl Video {
    fn new(a: &str) -> Self {
        Self(String::from(a))
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

fn main() {
    println!("Hello, world!");
    let a = Audio::new("audio");
    a.play();
    let b = Video::new("video");
    b.play();
}
