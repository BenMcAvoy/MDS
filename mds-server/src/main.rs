use crop::{RopeBuilder, Rope};
use std::sync::{Arc, Mutex};

// TODO: Load data instead of
// Empty roap being used.
#[derive(Default)]
struct MDSServer {
    rope: Arc<Mutex<Rope>>,
}

fn main() {
    let server = MDSServer::default();
}
