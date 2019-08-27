
mod custom_io;
mod monty_hall;
fn main() {
    println!("Welcome to Monty Hall! It looks a little rusty here...");

    loop {
        monty_hall::play_game();
        if !custom_io::read_yes_no("Play Again? ", Some(true)) {
            break;
        }
    }
}
