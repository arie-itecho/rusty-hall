use rand::{thread_rng, Rng};
use std::ops;

use crate::custom_io;

mod doors;
use doors::Doors;

#[derive(Default)]
pub struct Counter {
    switch: u64,
    stay: u64,
}

impl Counter {
    pub fn print(&self) {
        let total = self.switch + self.stay;
        if total == 0 {
            println!("No games played yet...");
            return;
        }
        let switch_percentage = (self.switch as f64 / total as f64) * 100.0;
        let stay_percentage = (self.stay as f64 / total as f64) * 100.0;

        println!("Total games: {}", total);
        println!(
            "It was the chosen door {} times. ({}%)",
            self.stay, stay_percentage
        );
        println!(
            "It was the other door {} times. ({}%)",
            self.switch, switch_percentage
        );
    }
}

impl ops::Add<Counter> for Counter {
    type Output = Counter;

    fn add(mut self, rhs: Counter) -> Counter {
        self.stay += rhs.stay;
        self.switch += rhs.switch;

        return self;
    }
}

pub fn play_game(counter: &mut Counter) {
    let mut doors = Doors::new();
    doors.set_winner();

    println!("Two doors contain (g)arabage, the other a (p)rize. You get what you choose...");

    let prompt = "Choose a door, any door (Enter 1, 2, or 3):";
    let choice = custom_io::read_input(prompt, &[1, 2, 3]);

    doors.choose_door(choice);
    println!("You picked door number {}.", choice);
    doors.print();

    custom_io::build_suspense("Now let me open another door", 3);
    doors.open_door();
    doors.print();

    let switch = custom_io::read_yes_no("Do you want to switch? (y/n)", None);
    if switch {
        doors.switch();
    }
    doors.print();

    custom_io::build_suspense("Let's see if you won", 3);
    let won = doors.resolve();
    doors.print();

    if won && switch {
        println!("YOU WON! Well done for understanding probabilities ;)");
        counter.switch += 1;
    }
    if won && !switch {
        println!("YOU WON! You may be stubborn, but at least you are lucky!");
        counter.stay += 1;
    }
    if !won && switch {
        println!("You lost. Bad luck :(");
        counter.stay += 1;
    }
    if !won && !switch {
        println!("You lost. You should've switched.");
        counter.switch += 1;
    }
}

pub fn simulate(counter: Counter) -> Counter {
    let test = |p| p >= 2 && p <= 100000;
    let runs = custom_io::read_validated_int("Enter a number between 2 and 100 000", test);

    let print_all = runs < 1000;
    let skip_counts = runs > 10001;

    if !print_all {
        println!("Not printing details. If you want to see details, pick a number below 1000.");
    }

    let mut simstats: Counter = Default::default();
    for i in 1..runs + 1 {
        if !skip_counts || i % 100 == 0 || i == runs {
            println!("Simulation {}{}", i, if print_all { ":" } else { "." });
        }

        let mut doors = Doors::new();
        let winner = doors.set_winner();
        let choice = thread_rng().gen_range(1, 4);
        doors.choose_door(choice);

        if print_all {
            println!("  Player chooses door {}.", choice);
            println!("  Presenter reveals door {}.", doors.open_door());
            println!("  The winning door is {}.", winner);
        }

        if winner == choice {
            simstats.stay += 1;
            if print_all {
                println!("  Player should stay.");
            }
        } else {
            simstats.switch += 1;
            if print_all {
                println!("  Player should switch.");
            }
        }
    }

    println!("Simulation Done:");
    simstats.print();
    counter + simstats
}