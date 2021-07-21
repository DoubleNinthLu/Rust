use std::cell::{Cell, RefCell};
use std::rc::Rc;


fn main() {
    println!("u8 : {}", std::mem::size_of::<u8>());
    println!("f64 : {}", std::mem::size_of::<f64>());
    println!("4u8 : {}", std::mem::size_of_val(&4u8));
    println!("4 : {}", std::mem::size_of_val(&4));
    println!("'a' : {}", std::mem::size_of_val(&'a'));

    println!("\"Hello World\" : {}", std::mem::size_of_val("Hello World"));
    println!("\"Hello World\" : {}", std::mem::size_of_val(&("Hello World").to_string()));
    println!("Cell(4) : {}", std::mem::size_of_val(&Cell::new(84)));
    println!("RefCell(4) : {}", std::mem::size_of_val(&RefCell::new(4)));

    println!("Rc(4) : {}", std::mem::size_of_val(&Rc::new(4)));
    println!("Rc<RefCell(8)> : {}", std::mem::size_of_val(&Rc::new(RefCell::new(4))));
}
