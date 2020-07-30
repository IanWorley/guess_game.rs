use rand::prelude::*;
use std::io;
fn main() {
    let x: i64 = gen_random_number(30, 40);
    let mut input = String::new();
    println!("Give me a number.");
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("We got your input.");
        }

        Err(error) => println!("Error {}", error),
    }

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input == x)
}

fn gen_random_number(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let x: i64 = rng.gen_range(low, high);
    return x;
}
