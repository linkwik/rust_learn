extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0,101);

    println!("                                    _                 ");
    println!("                                   (_)                ");
    println!("  __ _   _   _    ___   ___   ___   _   _ __     __ _ ");
    println!(" / _` | | | | |  / _ \\ / __| / __| | | | '_ \\   / _` |");
    println!("| (_| | | |_| | |  __/ \\__ \\ \\__ \\ | | | | | | | (_| |");
    println!(" \\__, |  \\__,_|  \\___| |___/ |___/ |_| |_| |_|  \\__, |");
    println!("  __/ |                                          __/ |");
    println!("|___/                                          |___/ ");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
   
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
