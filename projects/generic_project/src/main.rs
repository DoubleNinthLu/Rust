fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("{}", generic_largest(&number_list));
    println!("{:?}", number_list);

    let null_list = [102, 34, 6000, 89, 54];

    println!("{}", generic_largest(&null_list));
    println!("{:?}", null_list);

    let wont_work = Point { x: 1, y: 4 };
}

fn largest(list: &[i32]) -> i32 {
    // 内置类型，自动在栈内复制，因此largest拥有单独的所有权且可变；
    // largest 和 list[0] 所有的不是同一个栈内值；
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = list.get(0).expect("none list");

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{}", self.x);
        } else {
            println!("{}", self.y);
        }
    }
}

use std::fmt::Display;
