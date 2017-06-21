use std::process::Command;

#[cfg(windows)]
mod execs {
    pub const NPM: &'static str = "npm.cmd";
}
#[cfg(not(windows))]
mod execs {
    pub const NPM: &'static str = "npm";
}

fn main() {
    let status = Command::new(execs::NPM)
        .arg("-v")
        .output();

    println!("Command exited with '{:?}'", status);
    let status = status.expect("Process spawn should succeed!");
    if !status.status.success() {
        panic!("Command failed!");
    }
}
