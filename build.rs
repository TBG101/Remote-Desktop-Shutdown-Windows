use embed_resource;
use std::fs;

fn main() {
    embed_resource::compile("app_icon.rc", embed_resource::NONE);

    fs::copy("icon.ico", "target/release/icon.ico").expect("Failed to copy icon file");
}
