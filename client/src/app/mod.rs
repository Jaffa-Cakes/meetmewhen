use api_types::prelude::*;

use std::collections::HashMap;

use prokio::spawn_local as wait;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use root::Routes;

mod atoms;
mod components;
mod pages;
pub mod root;
