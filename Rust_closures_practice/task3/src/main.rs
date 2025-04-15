fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Implementation using map and collect
    vec.into_iter().map(f).collect()
    
    // Alternative implementation using for loop:
    /*
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
    */
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 { 0 } else { x }
    });

    println!("Doubled: {:?}", doubled); // Output: Doubled: [2, 4, 6]
    println!("Replaced: {:?}", replaced); // Output: Replaced: [1, 2, 0]
}