fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total); // Should print: "Total: 5050"
}