fn main() {
    let bag = Bag{food: Food::Cake};
    match &bag.food {
        Food::Cake => println!("I got cake"),
        a => println!("I got {:?}", a),
    }
    println!("{:?}", bag);
}

#[derive(Debug)]
enum Food {
    Cake,
    Pizza, 
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

