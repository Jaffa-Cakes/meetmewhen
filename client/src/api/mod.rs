use tonic_web_wasm_client::Client as WebClient;
use api_types::prelude::*;

pub use basic_event::Service as BasicEvent;

pub mod basic_event;
