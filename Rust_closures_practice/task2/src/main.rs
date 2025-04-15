fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Counter: {}", tracker);
    };

    update(); // Output: Counter: 1
    update(); // Output: Counter: 2
}

fn main() {
    track_changes();
}