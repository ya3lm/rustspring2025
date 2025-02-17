fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 42;
    let mut guess_count = 0;

    loop {
        // Simulate user input (hard-coded guesses for demonstration)
        let guess = 30 + guess_count; // Example: guesses start at 30 and increment

        guess_count += 1;

        // Check the guess
        let result = check_guess(guess, secret_number);

        // Print feedback
        if result == 0 {
            println!("Correct! You guessed the secret number.");
            break;
        } else if result == 1 {
            println!("Too high! Try again.");
        } else {
            println!("Too low! Try again.");
        }
    }

    println!("It took you {} guesses to find the secret number.", guess_count);
}