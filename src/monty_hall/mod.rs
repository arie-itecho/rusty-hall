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
        let switch_percentage = (self.switch / total) * 100;
        let stay_percentage = (self.stay / total) * 100;

        println!("Total games: {}", total);
        println!("It was the chosen door {} times. ({}%)", self.stay, stay_percentage);
        println!("It was the other door {} times. ({}%)", self.switch, switch_percentage);
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

    let switch = custom_io::read_yes_no("Do you want to switch?", None);
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