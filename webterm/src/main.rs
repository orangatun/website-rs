#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        link { rel: "stylesheet", href: "main.css" },
        div { id:"body", padding: "0.5rem", position: "relative",
            div { font_size: "1.5rem",
                p { "Welcome to Raghu's Terminal! (v0.1.0)" }
            }
        }
    }
}
