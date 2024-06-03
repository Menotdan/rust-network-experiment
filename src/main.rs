mod network;
mod types;
mod core;

// CLIENT
#[cfg(not(feature = "server"))]
fn main() {
    use core::client::init;
    init();
}

// SERVER
#[cfg(feature = "server")]
fn main() {
    use core::server::init;
    init();
}