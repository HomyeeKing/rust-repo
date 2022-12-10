use std::process::Command;

fn main() {
    match Command::new("git status").output() {
        Ok(url) => {
            println!("{:?}", String::from_utf8(url.stdout));
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
