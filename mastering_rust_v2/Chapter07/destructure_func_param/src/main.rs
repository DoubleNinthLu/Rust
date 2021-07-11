struct Container {
    items_count: u32,
}

fn incremetn_item(Container {mut items_count}: &mut Container) {
    items_count += 1;
}

fn calculate_cost(Container {items_count} : &Container) ->u32 {
    let rate = 67;
    rate * items_count
}
fn main() {
    println!("Hello, world!");
}
