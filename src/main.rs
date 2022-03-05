use std::fs;

fn main() {
    let dotfile = { std::env::var("HOME").unwrap() } + "/.fixpermissions";

    let contents = fs::read_to_string(dotfile).expect(
        "[ERROR] Dotfile unreachable: create a '.fixpermissions' dotfile in your home \
        directory and add to it the paths of your documents folders to fix.",
    );

    println!("{}", contents);
}
