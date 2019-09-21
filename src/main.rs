mod custom_io;
mod monty_hall;

fn main() {
    println!("Welcome to Monty Hall! It looks a little rusty here...");
    let mut counter : monty_hall::Counter = Default::default();

    loop {
        let menu = format!(
            "{}\n{}\n{}\n{}\n{}",
            "What do you want to do?",
            " - (p)lay",
            " - Simulate (m)any games",
            " - Print (s)tats",
            " - (q)uit"
        );

        let response = custom_io::read_str(&menu, &["p", "m", "s", "q"]);
        match &response[..] {
            "p" => monty_hall::play_game(&mut counter),
            "m" => continue, // TODO: Simulate many games
            "s" => counter.print(),
            "q" => break,
            _ => continue,
        }
    }
}
