// printing with string interpolation

use std::f64::consts;

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {

    println!("OK, value of PI is {}", consts::PI);
 
    println!();
    println!("The game you are playing is called {}.", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);

    println!();
    println!("In the Game {0} you start with {1}% health, and {1} points!",
             GAME_NAME, MAX_HEALTH);
}
