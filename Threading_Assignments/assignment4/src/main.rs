use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // Producer threads
    let tx1 = tx.clone();
    let producer1 = thread::spawn(move || producer(1, tx1, ITEM_COUNT / 2));
    
    let tx2 = tx.clone();
    let producer2 = thread::spawn(move || producer(2, tx2, ITEM_COUNT / 2));
    
    // Consumer threads
    let mut consumers = vec![];
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        consumers.push(thread::spawn(move || consumer(id, rx_clone)));
    }
    
    // Wait for producers
    producer1.join().unwrap();
    producer2.join().unwrap();
    
    // Send termination signals (one per consumer)
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    
    // Wait for consumers
    for consumer in consumers {
        consumer.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("Producer {} finished", id);
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        
        if num == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }
        
        println!("Consumer {} received: {}", id, num);
        thread::sleep(Duration::from_millis(150));
    }
}