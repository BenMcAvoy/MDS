#![deny(unsafe_code)]

use crop::{Rope, RopeBuilder};

use std::fs;
use std::io::Read;
use std::process;
use std::sync::{Arc, Mutex};

/// Where to store the rope
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
        // Get a file desccriptor to the rope
        // store path and create it if it does
        // not exist.
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(ROPE_STORE_PATH);

        // Check for errors when getting the
        // file descriptor.
        let mut file = match file {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to open or create file: {e}");
                process::exit(-1);
            }
        };

        // Read the rope store contents using
        // the file descriptor.
        let mut rope_contents = String::new();
        if let Err(e) = file.read_to_string(&mut rope_contents) {
            eprintln!("Error reading file: {e}");
            process::exit(-1);
        }

        // NOTE: `clone` does not matter much
        // here as we are in startup.
        let rope = RopeBuilder::new().append(rope_contents).clone().build();

        let rope = Arc::new(Mutex::new(rope));

        Self { rope }
    }
}
