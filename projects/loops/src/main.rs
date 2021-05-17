fn main() {
    let mut counter = 0;
    let a = [3; 5];

    while counter < 5 {
        print!("{}", a[counter]);
        counter += 1;
    }

    println!();

    for ele in a.iter() {
        print!("{}", ele);
    }

    println!();


    for ele in (1..4).rev() {
        print!("{}", ele);
    }
}
