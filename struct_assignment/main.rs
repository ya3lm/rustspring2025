use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    sid: String, // Changed from `api_key` to `sid`
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let sid = lines.next().unwrap().to_string(); // Read SID instead of API key

        Config { name, sid }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.name);
    println!("SID: {}", config.sid); // Print SID instead of API key
}

fn main() {
    reading_from_file();
}