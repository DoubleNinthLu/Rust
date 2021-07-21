use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    let join_handle = thread::spawn(move || {
        println!("{:?}", rx.recv());
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
        println!("{:?}", rx.recv());
    });

    for i in 0..10 {
        tx.send(i).unwrap();
    }

    join_handle.join().unwrap();
}
