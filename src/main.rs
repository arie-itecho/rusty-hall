use std::fmt::Display;
use std::io;
use std::str::FromStr;

mod monty_hall;
use monty_hall::Doors;

fn read_input<T: FromStr + PartialEq>(message: &str, accept: &[T]) -> T
where
    <T as FromStr>::Err: Display,
{
    let mut input: Option<T> = None;
    while input.is_none() || !accept.contains(input.as_ref().unwrap()) {
        println!("{}", message);

        let mut in_str = String::new();
        match io::stdin().read_line(&mut in_str) {
            Ok(_) => {}
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        }

        input = match in_str.trim().parse::<T>() {
            Ok(x) => Some(x),
            Err(error) => {
                println!("{}", error);
                None
            }
        }
    }

    input.unwrap()
}

fn main() {
    println!("Welcome to Monty Hall! It looks a little rusty here...");

    loop {
        let mut doors = Doors::new();
        doors.set_winner();

        println!("Two doors contain (g)arabage, the other a (p)rize. You get what you choose...");

        let prompt = "Choose a door, any door (Enter 1, 2, or 3):";
        let choice = read_input(prompt, &[1, 2, 3]);

        doors.choose_door(choice);
        println!("You picked door number {}.", choice);
        doors.print();

        break;
    }
}
