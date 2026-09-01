// ═══ CHAPTER 02: GUESSING GAME ═══
// Topics: control flow, loops, Result, Option, std::io

use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Guess the number (1-100)!");
    println!("Type 'quit' to exit.\n");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        attempts += 1;
        print!("Attempt {}: > ", attempts);

        // Flush stdout so the prompt appears before reading
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        // Quit
        if input == "quit" {
            println!("Bye! The number was {}.", secret);
            break;
        }

        // Parse
        let guess: u32 = match input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("⚠️  Please enter a number or 'quit'.");
                continue;
            }
        };

        // Compare
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less    => println!("📈 Too small!\n"),
            std::cmp::Ordering::Greater => println!("📉 Too big!\n"),
            std::cmp::Ordering::Equal   => {
                println!("🎉 You got it in {} attempts!", attempts);
                break;
            }
        }
    }
}

// flush helper
use std::io::Write;