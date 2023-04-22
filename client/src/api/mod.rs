use api_types::prelude::*;
use tonic_web_wasm_client::Client as WebClient;

pub use availabilities::Service as Availabilities;
pub use basic_event::Service as BasicEvent;

pub mod availabilities;
pub mod basic_event;
