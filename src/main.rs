use std::io;
use std::time::Instant;

fn main() {
    println!("Welcome to the Number Guessing Game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::random::<u32>() % 100 + 1;

    println!("I have generated a random number between 1 and 100.");

    // The algorithm you want to test
    linear_search(secret_number);
    binary_search(secret_number);
}

fn get_guess() -> u32 {
    loop {
        let mut guess = String::new();

        println!("Please enter your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number
        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        }
    }
}

fn linear_search(secret_number: u32) {
    println!("You are using Linear Search algorithm.");

    let start_time = Instant::now();
    let mut attempts = 0;

    loop {
        // let guess = get_guess();
        let guess = rand::random::<u32>() % 100 + 1;
        attempts += 1;

        println!("You guessed: {}", guess);

        if guess == secret_number {
            let elapsed_time = start_time.elapsed();
            println!(
                "Congratulations! You guessed the correct number in {} attempts. Elapsed time: {:?}",
                attempts,
                elapsed_time
            );
            break;
        } else if guess < secret_number {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
    }
}

fn binary_search(secret_number: u32) {
    println!("You are using Binary Search algorithm.");

    let start_time = Instant::now();
    let mut attempts = 0;
    let mut low = 1;
    let mut high = 100;

    loop {
        let guess = (low + high) / 2;
        // let guess = get_guess();
        attempts += 1;

        println!("You guessed: {}", guess);

        if guess == secret_number {
            let elapsed_time = start_time.elapsed();
            println!(
                "Congratulations! You guessed the correct number in {} attempts. Elapsed time: {:?}",
                attempts,
                elapsed_time
            );
            break;
        } else if guess < secret_number {
            println!("Too low! Try again.");
            low = guess + 1;
        } else {
            println!("Too high! Try again.");
            high = guess - 1;
        }
    }
}