#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::collections::HashMap;
use chrono::Local;
use std::sync::OnceLock;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(app);
}

enum Errors {
    CommandNotFound,
    ExtraParametersPassed,
    PathNotFound,
    FileNotFound
}

struct TerminalEntryData {
    req: String,
    resp: Element
}

fn fs() -> &'static HashMap<&'static str, &'static [&'static str; 4]> {
    static FS: OnceLock<HashMap<&str,   &'static [&'static str; 4]>> = OnceLock::new();
    FS.get_or_init(|| {
        let mut fs = HashMap::from([
            ("/", &["about/", "projects/", "work/", "random/"]),
            ("/about/", &["summary", "secrets/", "", ""]),
            ("/about/secrets/", &["shhh", "", "", ""]),
            ("/projects/", &["summary", "", "", ""]),
            ("/random/", &["warp", "rules_of_internet", "", ""])
        ]);
        fs
    })
}

#[component]
fn app() -> Element {

    let mut current_path =use_signal(|| HashMap::from([(0u8,"/".to_string())]));
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
                    TerminalEntry { key: "{id}", id, entries, current_path}
                }
            },
            TerminalActiveEntry { entries , start_from, ending , current_path},
        }
    }
}

fn ls_response(words: Vec::<String>, current_path: Signal<HashMap::<u8, String>>) -> Element{
    let mut key: u8 = 0u8;
    let mut path: String = String::new();
    for i in 0..current_path().len() {
        path.push_str(&format!("{}",current_path().get(&key).unwrap()));
        key+=1u8;
    }
    let contents = **fs().get(path.as_str()).unwrap();

    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div {
            if path!="/" {
                div {
                    class: "ls-item-container",
                    span { class: "directory",
                        ".."
                    }
                }
            } 
            div {
                class: "ls-item-container",
                span { class: "directory",
                    "."
                }
            }
            for i in contents {
                if !i.is_empty() {
                    div { class: "ls-item-container",
                        span { class: if i.ends_with("/") { "directory"} else {"file"},
                            "{i}"
                        },
                    }
                }
            }
        }
    }
}

fn cat_dog_response(words: Vec::<String>) -> Element{
    rsx! {
        div {
            "Cat/Dog response",
        }
    }
}


fn cd_response(words: Vec::<String>, mut current_path: Signal<HashMap::<u8, String>>) -> Element{
    if words.len()==1 {
        for i in 1..current_path().len() {
            let key = i as u8;
            current_path().remove(&key);
        }
    } else {
        //Path check, and change
    }
    
    rsx! {    
        div {
            "cd response here",
        }
    }
}

fn resolve_error(err: Errors) -> Element {
    rsx! {
        div {
            "Error resolved here"
        }
    }
}

fn resolve_exit() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div { id: "exit-container",
            p { "There's no exiting to this terminal." },
            p { "The only way out is turning off the internet." }
            p { class: "whisper-text", "Or, you could close this tab." }
        }
    }
}

fn generate_response(req: String, current_path: Signal<HashMap::<u8, String>>) -> Element {
    let words: Vec<String> = req.trim().split_whitespace().map(|v| v.to_string()).collect();

    if words.len()==0 {
        return resolve_error(Errors::CommandNotFound);
    } else {
        let first_word = words.first().unwrap();
        match first_word.as_str() {
            "ls" => {
                if words.len()>1 {
                    return resolve_error(Errors::ExtraParametersPassed);
                } else {
                    return ls_response(words, current_path);
                }
            },
            "cd" => {
                return cd_response(words, current_path);
            },
            "pwd" => {
                let p_len = (current_path().len() as u8)-1u8;
                return rsx! {
                        div {
                            "The current working directory is: {current_path().get(&p_len).unwrap()}"
                        }
                    }
            }, 
            "cat" | "dog" => {
                return cat_dog_response(words);
            },
            "exit" => {
                return resolve_exit();
            }
            _ => {
                return resolve_error(Errors::CommandNotFound);
            }
        }
    }
}

#[component]
fn TerminalActiveEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, mut start_from: Signal<u32>, mut ending: Signal<u32>, mut current_path: Signal<HashMap::<u8, String>>) -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut entry_id = use_signal(|| 0);

    let onkeyup = move |event: KeyboardEvent| {
        if event.key() == Key::Enter && !draft.read().is_empty() {
            let tmp_str = draft.to_string();
            if tmp_str=="clear" {
                entries.write().clear();
                start_from-=start_from();
                ending -=ending();
                entry_id -=entry_id();
            } else {
                let id = entry_id();
                let entry = TerminalEntryData {
                    req: draft.to_string(),
                    resp: generate_response(draft.to_string(), current_path),
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
fn TerminalEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, id: u32, current_path: Signal<HashMap::<u8, String>>) -> Element {
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
            { resp }
        }
    }
}