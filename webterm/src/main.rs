#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::collections::HashMap;
use chrono::Local;


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app);
}

struct TerminalEntryData {
    req: String,
    resp: String
}

#[component]
fn app() -> Element {
    let mut entries = use_signal(HashMap::<u32, TerminalEntryData>::new);

    let mut start_from = use_signal(||0);
    let mut ending = use_signal(||0);

    let date_string = format!("{}", Local::now().format("%a, %m/%d/%Y, %l:%M:%S %p UTC%Z"));

    rsx! {
        link { rel: "stylesheet", href: "main.css" },
        div { class:"container", padding: "0.5rem", position: "relative",
            div { font_size: "1.5rem",
                p { "Welcome to Raghu's Terminal! (v0.1.0)" },
                p { "{date_string}" },
            }
        }, 
        div {
            ul {
                class: "entries-list",
                for id in start_from()..ending() {
                    TerminalEntry { key: "{id}", id, entries}
                }
            },
            TerminalActiveEntry { entries , start_from, ending },
        }
    }
}

fn generate_response(req: String) -> String {
    format!("Response to: {}", req)
}

#[component]
fn TerminalActiveEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, mut start_from: Signal<u32>, mut ending: Signal<u32>) -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut entry_id = use_signal(|| 0);

    let onkeyup = move |event: KeyboardEvent| {
        if event.key() == Key::Enter && !draft.read().is_empty() {
            let tmp_str = draft.to_string();
            if(tmp_str=="clear") {
                entries.write().clear();
                start_from-=start_from();
                ending -=ending();
                entry_id -=entry_id();
            } else {
                let id = entry_id();
                let entry = TerminalEntryData {
                    req: draft.to_string(),
                    resp: generate_response(draft.to_string()),
                };
                ending += 1;
                entries.write().insert(id, entry);
                entry_id += 1;
            }
                draft.set("".to_string());
        }
    };

    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        span {
            class:"prompt", ">"
        },
        input {
            value: "{draft}",
            autofocus: "true",
            oninput: move |event| draft.set(event.value()),
            onkeyup
        }
    }
}

#[component]
fn TerminalEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, id: u32) -> Element {
    let req = entries.read().get(&id).unwrap().req.clone();
    let resp = entries.read().get(&id).unwrap().resp.clone();

    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div {
            span {
                class:"prompt", ">"
            },
            p { 
                class:"req",
                "{req}"
            },
            p {
                class:"resp",
                "{resp}"
            }
        }
    }
}