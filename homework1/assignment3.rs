fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Hard-coded secret number
    let secret_number = 42;
    let mut guess_count = 0; // Track the number of guesses

    // Loop until the correct guess is made
    loop {
        // Simulate user input 
        let guess = 30 + guess_count; 

        guess_count += 1; // Increment guess count

        // Check the guess using the check_guess function
        let result = check_guess(guess, secret_number);

        // Print feedback based on the result
        if result == 0 {
            println!("Correct! You guessed the secret number.");
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Too high! Try again.");
        } else if result == -1 {
            println!("Too low! Try again.");
        }
    }

    // Print the total number of guesses
    println!("It took you {} guesses to find the secret number.", guess_count);
}