// My first rustlang project
// The Guessing game!

// the way to include external libraries as dependencies
// here rand is the external library
extern crate rand;

// use is the analogu for import, to add to the namespace
// std is the main module, io is a submodule, and stdin is a method defined inside io and so on.

// The prelude is the list of things that Rust automatically imports into every Rust program.
// prelude is kept at min
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // generate a random number between 0 -100 (0 <= secret < 100)
    let secret: u32 = rand::thread_rng().gen_range(0, 100);

    println!("Hey there, enter your name:");

    // By default, all the instances in Rust are immutable
    // need to explicitly say `mut` to make the value mutable
    // need to make the String mutable in order to update it with the value
    // from the stdin.
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("PLease enter a valid name lol!");

    if name.contains("Si") {

        println!("So your name is {}?", &name);
    } else {

        println!("Nice name you got there!");
    }

    println!("{}, enter a number [0, 100):", name);

    loop {

        let mut number: String = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Please enter a valid whole number");

        let number: u32 = match number.trim().parse() {

            Ok(number) => number,
            Err(_) => continue,
        };

        match number.cmp(&secret) {
            Ordering::Equal => {
                println!("Well done, {}. You won!", &name);
                break;
            }
            Ordering::Less => println!("Guess higher, {}", &name),
            Ordering::Greater => println!("Guess lower, {}", &name),
        };
    }

}
