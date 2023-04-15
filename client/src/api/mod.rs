use api_types::prelude::*;

#[cfg(target_arch = "wasm32")]
use tonic_web_wasm_client::Client as WebClient;

pub use basic_event::Service as BasicEvent;

pub mod basic_event;
