use std::fs;

fn main() {
    fs::copy("icon.ico", "target/release/icon.ico").expect("Failed to copy icon file");
}
