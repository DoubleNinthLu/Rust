fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("{}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the back");
        } else {
            println!("Using orange as the back");
        }
    } else {
        println!("Using blue as the back");
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_v @ 3..=7 } => {
            println!("1 {}", id_v);
        }

        Message::Hello { id: id_v @ 10..=12 } => {
            println!("2 {}", id_v);
        }
        Message::Hello { id } => {
            println!("3 {}", id);
        }
    }
}

enum Message {
    Hello { id: i32 },
}
