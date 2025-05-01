use std::process::Command;

fn main(){
    let output = Command::new("ls")
    .arg("-1")
    .arg("my_files")
    .output() //exec happens here
    .expect("Failed to execute command");

    let _new_file_name =

    println!("Command output: {}",
    String::from_utf8_lossy(&output.stdout));
}