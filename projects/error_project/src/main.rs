use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let file_content = read_username_from_file().expect("test");
    println!("{}", file_content);

    println!("{}", read_username_from_file_simple().unwrap());

    let p = test_custom_type::Guess::new(-1);
    println!("{}", p.value());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

mod test_custom_type {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("error");
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
