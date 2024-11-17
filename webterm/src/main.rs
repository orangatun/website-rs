#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use chrono::Local;


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
        div { class:"container", padding: "0.5rem", position: "relative",
            div { font_size: "1.5rem",
                p { "Welcome to Raghu's Terminal! (v0.1.0)" },
                span { "{dateString}" },
            }
        },
        TerminalPrompt {},
    }
}

fn generate_response(req: String) -> String {
    format!("Response to: {}", req)
}

fn TerminalPrompt() -> Element {
    let mut command = use_signal(|| "".to_string());
    let mut final_resp = use_signal(|| "".to_string());
    let mut is_disabled = use_signal(|| false);
    let submit_evt = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter { 
            is_disabled.set(true);
            final_resp.set(generate_response(command()));
        }
    };


    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div {
            span {
                class:"prompt", ">"
            },
            input {
                disabled: is_disabled,
                "type": "text",
                value: "{command}",
                oninput: move |event| {
                    command.set(event.value());
                },
                onkeyup: submit_evt
            },
            p {
                "{final_resp}"
            }
        }
    }
}