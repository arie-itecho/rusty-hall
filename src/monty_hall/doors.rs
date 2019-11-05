use rand::{thread_rng, Rng};

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
pub struct Doors {
    doors: Vec<i8>,
}

impl Doors {
    pub fn new() -> Doors {
        Doors {
            doors: vec![0, 0, 0],
        }
    }

    fn clean(&mut self, disallowed: i8) {
        let allowed = WINNER + OPEN + CHOSEN - disallowed;
        self.doors = self.doors.iter().map(|d| d & allowed).collect();
    }

    pub fn set_winner(&mut self) -> usize {
        self.clean(WINNER);

        let winner: usize = thread_rng().gen_range(0, self.doors.len());
        self.doors[winner] |= WINNER;
        winner + 1
    }

    pub fn choose_door(&mut self, choice: usize) {
        self.clean(CHOSEN);

        let i = choice - 1;
        self.doors[i] |= CHOSEN;
    }

    pub fn open_door(&mut self) -> usize {
        self.clean(OPEN);

        let mut choices: Vec<usize> = Vec::new();
        for (i, door) in self.doors.iter().enumerate() {
            if door & (CHOSEN + WINNER) == 0 {
                choices.push(i);
            }
        }

        let chosen: usize = choices[thread_rng().gen_range(0, choices.len())];
        self.doors[chosen] |= OPEN;

        return chosen + 1;
    }

    pub fn switch(&mut self) {
        let mut new_choice = self.doors.len();
        for (i, door) in self.doors.iter().enumerate() {
            if door & OPEN + CHOSEN == 0 {
                new_choice = i;
                break;
            }
        }
        self.clean(CHOSEN);
        self.doors[new_choice] |= CHOSEN;
    }

    pub fn resolve(&mut self) -> bool {
        let mut won = false;
        for (i, door) in self.doors.clone().iter().enumerate() {
            if door & OPEN == 0 {
                self.doors[i] |= OPEN;
                if self.doors[i] == OPEN + CHOSEN + WINNER {
                    won = true
                }
            }
        }
        return won;
    }

    pub fn print(&self) {
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