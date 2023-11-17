#![deny(unsafe_code)]

use ropey::Rope;

use std::fs;
use std::io::BufReader;
use std::process;
use std::sync::{Arc, Mutex};

const ROPE_STORE_PATH: &str = "./rope.md";

/// The main struct for storing data in the
/// server and passing it around.
///
/// You can make a new server by using
/// ```
/// MDSServer::default();
/// ```
#[derive(Debug)]
pub struct MDSServer {
    pub(crate) rope: Arc<Mutex<Rope>>,
}

/// Create a new MDSServer with the contents
/// of the rope coming from `ROPE_STORE_PATH`.
impl Default for MDSServer {
    fn default() -> Self {
        // Create a file descriptor that will
        // create the file if it does not
        // already exist.
        let descriptor = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(ROPE_STORE_PATH);

        // Extract the okay variant from the
        // descriptor and fail if it can't.
        let descriptor = match descriptor {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to open or create file: {e}");
                process::exit(-1);
            }
        };

        // Create a rope.
        let rope = Rope::from_reader(BufReader::new(descriptor));

        // Wrap it in an Arc and a Mutex.
        let rope = Arc::new(Mutex::new(rope.unwrap()));

        Self { rope }
    }
}
