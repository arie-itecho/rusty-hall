use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Counter {
    switch: u64,
    stay: u64,
}

/// Doors have the following states:
///   0: closed (normal)
///   1: closed (prize)
///   2: open (garbage)
///   3: open (prize)
///   4: chosen (normal)
///   5: chosen (prize)
///   6: chosen (lost)
///   7: chosen (won)
pub struct Doors {
    doors: Vec<i8>,
    has_winner: bool,
}

impl Doors {
    pub fn new() -> Doors {
        Doors {
            doors: vec![0, 0, 0],
            has_winner: false,
        }
    }

    pub fn set_winner(&mut self) -> usize {
        if self.has_winner {
            panic!("This set of doors already has a winner!");
        }

        let winner: usize = thread_rng().gen_range(0, 3);
        self.doors[winner] |= 1;
        self.has_winner = true;
        winner + 1
    }

    pub fn choose_door(&mut self, choice: usize) {
        // make sure no other door is chosen
        self.doors = self.doors.iter().map(|d| d & 3).collect();

        let i = choice - 1;
        self.doors[i] |= 4;
    }

    pub fn print(&self)
    {
        for door in &self.doors{
            match door {
                0 | 1 => print!("[\u{25A0}]"),
                2 => print!("[g]"),
                3 => print!("[p]"),
                4 => print!("[*]"),
                5 => print!("[*]"),
                6 => print!("[G]"),
                7 => print!("[P]"),
                _ => panic!("Unknown door state!")
            }
        }
        println!();
    }
}
