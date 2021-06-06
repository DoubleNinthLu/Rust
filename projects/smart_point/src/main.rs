use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let value = RefCell::new(5);

    let a = Rc::new(Cons(value, Rc::new(Nil)));

    let b = Cons(RefCell::new(6), Rc::clone(&a));
    let c = Cons(RefCell::new(19), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
