use std::fmt::Display;
use std::io::{self, Write};

use std::str::FromStr;
use std::{thread, time};

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

pub fn read_yes_no(prompt: &str, default: Option<bool>) -> bool {
    loop {
        let response = read_str(prompt, &[]);

        let first_char = match response.chars().next() {
            Some(c) => c,
            None => {
                match default {
                    Some(d) => return d,
                    None => ' '
                }
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

pub fn wait_any_key() {
    read_str("Press any key", &[]);
}

pub fn build_suspense(message: &str, seconds: u8) {
    let flush = || io::stdout().flush().unwrap_or_default();

    print!("{}", message);
    flush();

    for _ in 0..seconds {
        print!(".");
        flush();
        thread::sleep(time::Duration::from_secs(1));
    }

    println!(".");
}

pub fn seperator(start: char, line: char, width: u32, end: char)
{
    print!("{}", start);
    for _ in 0..width {
        print!("{}", line);
    }
    println!("{}", end);
}