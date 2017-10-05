extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the launch code game!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("The super secret launch code is between 1 and 100...guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
          .expect("Failed to compute your codes dude.");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Those codez {} is too tiny!", guess),
            Ordering::Greater => println!("Those codez {} is too large!", guess),
            Ordering::Equal => {
              println!("Missles launched, Fatty McMissile Pants watch out!");
              break;
            }
        }
    }
}
