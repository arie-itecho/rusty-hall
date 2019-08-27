use std::fmt::Display;
use std::io;
use std::str::FromStr;

pub fn read_input<T: FromStr + PartialEq>(prompt: &str, accept: &[T]) -> T
where
    <T as FromStr>::Err: Display,
{
    let mut input: Option<T> = None;
    while input.is_none() || (accept.len() > 0 && !accept.contains(input.as_ref().unwrap())) {
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

pub fn read_str(prompt: &str, accept: &[&str]) -> String {
    let accept: Vec<String> = accept.iter().map(|s| String::from(*s)).collect();
    read_input(prompt, &accept)
}

pub fn read_yes_no(prompt: &str, default: bool) -> bool {
    loop {
        let response = read_str(prompt, &[]);

        let first_char = match response.chars().next() {
            Some(c) => c,
            None => {
                return default;
            }
        };

        if let Some(i) = first_char.to_lowercase().next() {
            if i == 'y' {
                return true;
            }
            if i == 'n' {
                return false;
            }
        };
    }
}
