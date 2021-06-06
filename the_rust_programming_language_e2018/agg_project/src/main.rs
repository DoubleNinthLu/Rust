use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    v.push(5);
    v.push(6);

    let third: &i32 = &v1[2];

    println!("{}", third);
    match v1.get(1) {
        Some(third) => println!("{}", third),
        None => println!("no"),
    }

    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let mut s = String::new();
    let data = "initial contents";
    let s1 = data.to_string();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    format!("{}-{}", s, s1);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores1);

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    let v = scores2.entry(String::from("Yellow")).or_insert(50);
    // scores2.entry(String::from("Blue")).or_insert(50);

    println!("{}", v);
    *v = 20;
    println!("{}", v);
    println!("{:?}", scores2);
}
