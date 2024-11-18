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
    DirectoryNotFound,
    FileNotFound,
    FileNotDirectory,
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

fn files() -> &'static HashMap<&'static str, &'static str> {
    static FILES: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    FILES.get_or_init(|| {
        let mut files = HashMap::from([
            ("/about/summary", "This project is built in Rust, using Dioxus, and some CSS."),
            ("/about/secrets/shh", "There's an `exit` command. Don't try that."),
            ("/projects/summary", "Let me talk about this one. I worked 4 days on building this after two years of not using Rust. I'd used Rust for 3 months before this, and I'd never heard of Dioxus before. Here we are. I'm a quick learner and I love to push myself to learn."),
            ("/random/warp", "I think Warp is pretty cool. I'd love to be a part of the team, and contribute to making it a great product."),
            ("/random/rules_of_internet", "It's silly, and mostly trolls. Of course, it's on a chan! Look up rule 16.")
        ]);
        files
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

fn build_abs_path(current_path: Signal<HashMap::<u8, String>>) -> String{
    let mut path: String = String::new();
    let mut key: u8 = 0u8;
    for i in 0..current_path().len() {
        path.push_str(&format!("{}",current_path().get(&key).unwrap()));
        key+=1u8;
    }
    path
}

fn ls_response(words: Vec::<String>, current_path: Signal<HashMap::<u8, String>>) -> Element{
    let mut key: u8 = 0u8;
    let path = build_abs_path(current_path);
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

fn cat_dog_response(words: Vec::<String>, mut current_path: Signal<HashMap::<u8, String>>) -> Element{
    let mut word: String = String::new();
    if words.len()>1 {
        for i in 1..words.len() {
            if words.get(i).unwrap().len()!=0 {
                if(word.len()>0) {
                    return resolve_error(Errors::ExtraParametersPassed);
                }
                word.push_str(words.get(i).unwrap());
            }
        }
    }

    let mut path: String = build_abs_path(current_path);
    path.push_str(word.as_str());
    let contents = files().get(path.as_str());
    match contents {
        None => {
            return build_element("File Not Found");
        },
        Some(val) => {
            return rsx! {
                div {
                    "{**val}"
                }
            }
        }
    }
}

fn build_element(st: &str) -> Element {
    rsx! {
        div {
            "{st}"
        }
    }
}


fn cd_response(words: Vec::<String>, mut current_path: Signal<HashMap::<u8, String>>) -> Element{
    let mut word: String = String::new();
    if words.len()>1 {
        for i in 1..words.len() {
            if words.get(i).unwrap().len()!=0 {
                if(word.len()>0) {
                    return resolve_error(Errors::ExtraParametersPassed);
                }
                word.push_str(words.get(i).unwrap());
            }
        }
    }

    if(words.len()==1 || word.len()==0) {
        for i in 1..current_path().len() {
            let key = i as u8;
            current_path().remove(&key);
        }
    } else {
        let parts : Vec::<&str> = word.trim().split("/").collect();
        if parts.len()>2 {
            return build_element("Only cd to parent or child directories are supported at the moment.")
        } else {
            match word.trim() {
                "." | "./" => return build_element("No change in directory"),
                ".." | "../" => {
                    if current_path().len()==1 {
                        return build_element("Already at root directory. Cannot go higher.");
                    } else {
                        let mut last_key: u8 = current_path().len() as u8;
                        last_key-=1u8;
                        current_path.write().remove(&last_key);
                        return build_element(format!("Path changed to {}", build_abs_path(current_path)).as_str());
                    }
                },
                _ => {
                    let pwd = build_abs_path(current_path);
                    let contents = **fs().get(pwd.as_str()).unwrap();
                    let dir: String = format!("{}/",*parts.get(0).unwrap());
                    let key: u8 = current_path().len() as u8;
                    if contents.contains(&dir.as_str()) {
                        current_path.write().insert(key, dir);
                        return build_element(format!("Path changed to {}", build_abs_path(current_path)).as_str());
                    } else {
                        return resolve_error(Errors::DirectoryNotFound);
                    }
                }
            }
        }
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

fn help_response() -> Element {

    rsx! {
        div {
            span { class: "help-command",
                "ls"
            }, 
            p { class: "help-desc",
                "Lists files and directories inside current directory."
            }
        },
        div {
            span { class: "help-command",
                "cd"
            }, 
            p { class: "help-desc",
                "Changes directory to the go up one level (parent), or down one level (child)."
            }
        },
        div {
            span { class: "help-command",
                "pwd"
            }, 
            p { class: "help-desc",
                "Lists the full path of current working directory."
            }
        },
        div {
            span { class: "help-command",
                "cat"
            }, 
            p { class: "help-desc",
                "Used to print the contents of a file."
            }
        },
        div {
            span { class: "help-command",
                "dog"
            }, 
            p { class: "help-desc",
                "Used to print the contents of a file."
            }
        },
        div {
            span { class: "help-command",
                "help"
            }, 
            p { class: "help-desc",
                "Displays this list of commands available."
            }
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
                            "The current working directory is: {build_abs_path(current_path)}"
                        }
                    }
            }, 
            "cat" | "dog" => {
                return cat_dog_response(words, current_path);
            },
            "help" => {
                return help_response();
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