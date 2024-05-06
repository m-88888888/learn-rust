use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{:?}", map);
    println!("{:?}", field_name);

    println!("{:?}", read_username_from_file());

    let x = String::from("hoge");
    let y = String::from("fugafuga");
    println!("The longest string is: {}", longest(&x, &y));
}

use std::fs::File;
use std::io::Read;
use std::io::{self, Error};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
