fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [3, 7, 12, 15, 20, 22, 30, 33, 40, 45];

    // Analyze numbers using a for loop
    for &n in numbers.iter() {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else if is_even(n) {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }

    // Find the sum of all numbers using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Find the largest number using a loop
    let mut largest = numbers[0];
    for &n in numbers.iter() {
        if n > largest {
            largest = n;
        }
    }
    println!("Largest number: {}", largest);
}