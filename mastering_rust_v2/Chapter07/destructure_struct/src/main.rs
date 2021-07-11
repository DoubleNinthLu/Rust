#[derive(Debug)]
enum Food {
    Pizza,
    Salad,
}

#[derive(Debug)]
enum PaymentMode {
    Bitcoin,
    Credit,
}

#[derive(Debug)]
struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode,
}

fn main() {
    let food_order = Order {
        count: 2,
        item: Food::Salad,
        payment: PaymentMode::Credit,
    };
    let Order {
        count: count1,
        item: _item,
        ..
    } = food_order;

    println!("{:?}", count1);
}
