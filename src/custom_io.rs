use std::fmt::Display;
use std::io;
use std::str::FromStr;

pub fn read_input<T: FromStr + PartialEq>(prompt: &str, accept: &[T]) -> T
where
    <T as FromStr>::Err: Display,
{
    let mut input: Option<T> = None;
    while input.is_none() || !accept.contains(input.as_ref().unwrap()) {
        println!("{}", prompt);

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