use std::thread;

fn main() {
    let child = thread::spawn(|| {
        println!("Thread!");
        "Much concurrent, such wow!".to_string()
    });
    print!("Hello ");
    let value = child.join().expect("Failed joining child thread");
    println!("{}", value);
}
