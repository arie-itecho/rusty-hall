use rand::{thread_rng, Rng};

use crate::custom_io;

#[derive(Default)]
pub struct Counter {
    switch: u64,
    stay: u64,
}

pub fn play_game() {
    let mut doors = Doors::new();
    doors.set_winner();

    println!("Two doors contain (g)arabage, the other a (p)rize. You get what you choose...");

    let prompt = "Choose a door, any door (Enter 1, 2, or 3):";
    let choice = custom_io::read_input(prompt, &[1, 2, 3]);

    doors.choose_door(choice);
    println!("You picked door number {}.", choice);
    doors.print();
}

// Door states
const WINNER: i8 = 1;
const OPEN: i8 = 2;
const CHOSEN: i8 = 4;

/// Doors have the following states:
///     0: closed (normal)
///     1: closed (prize)
///     2: open (garbage)
///     3: open (prize)
///     4: chosen (normal)
///     5: chosen (prize)
///     6: chosen (lost)
///     7: chosen (won)
struct Doors {
    doors: Vec<i8>,
}

impl Doors {
    fn new() -> Doors {
        Doors {
            doors: vec![0, 0, 0],
        }
    }

    fn clean(&mut self, disallowed: i8) {
        let allowed = WINNER + OPEN + CHOSEN - disallowed;
        self.doors = self.doors.iter().map(|d| d & allowed).collect();
    }

    fn set_winner(&mut self) -> usize {
        self.clean(WINNER);

        let winner: usize = thread_rng().gen_range(0, 3);
        self.doors[winner] |= WINNER;
        winner + 1
    }

    fn choose_door(&mut self, choice: usize) {
        self.clean(CHOSEN);

        let i = choice - 1;
        self.doors[i] |= CHOSEN;
    }

    fn print(&self) {
        for door in &self.doors {
            match door {
                0 | 1 => print!("[\u{25A0}]"),
                2 => print!("[g]"),
                3 => print!("[p]"),
                4 => print!("[*]"),
                5 => print!("[*]"),
                6 => print!("[G]"),
                7 => print!("[P]"),
                _ => panic!("Unknown door state!"),
            }
        }
        println!();
    }
}