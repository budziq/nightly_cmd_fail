use std::process::Command;

fn main() {
    let status = Command::new("npm.cmd")
        .arg("-v")
        .output();

    println!("Command exited with '{:?}'", status);
    let status = status.expect("Process spawn should succeed!");
    if !status.status.success() {
        panic!("Command failed!");
    }
}
