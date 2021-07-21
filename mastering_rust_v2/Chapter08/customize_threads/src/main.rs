use std::thread;

fn main(){
    let my_thread = thread::Builder::new().name("Lucy Thread".to_string()).stack_size(1024 * 4);
    let handle = my_thread.spawn(|| {
        panic!("Oops!");
    }).unwrap();
    let child_status = handle.join();
    println!("Child status: {:?}", child_status);
}
