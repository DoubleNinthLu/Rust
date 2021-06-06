// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        mod back_of_house {
            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            pub enum Appetizer {
                Soup,
                Salad,
            }

            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }

            fn cook_order() {}
        }

        pub fn eat_at_restaurant() {
            let mut meal = back_of_house::Breakfast::summer("Rye");
            meal.toast = String::from("Wheat");
            println!("{}", meal.toast);

            // meal.seasonal_fruit = String::from("blueberries");
        }

        pub fn eat_at_restaurant_enum() {
            let order1 = back_of_house::Appetizer::Soup;
            let order2 = back_of_house::Appetizer::Salad;
        }

        fn take_payment() {}
    }
}

// 使用 use 关键字将 绝对路径 引入作用域
// use crate::front_of_house::hosting;

// 使用 use 关键字来将 相对路劲 引入作用域
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用use
    hosting::add_to_waitlist();
}
