#[cfg(windows)]
fn main() {
    embed_resource::compile("src/time-app.manifest");
}

#[cfg(not(windows))]
fn main() {
    // Do nothing...
}