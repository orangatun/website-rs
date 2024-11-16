#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use chrono::{Local, Datelike};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {

    let dt = Local::now();
    let dateString = format!("{}", dt.format("%a, %m/%d/%Y, %l:%M:%S %p UTC%Z"));
    rsx! {
        link { rel: "stylesheet", href: "main.css" },
        div { id:"body", padding: "0.5rem", position: "relative",
            div { font_size: "1.5rem",
                p { "Welcome to Raghu's Terminal! (v0.1.0)" },
                p { "{dateString}" }
            }
        }
    }
}
