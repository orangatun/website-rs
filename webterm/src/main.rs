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

#[derive(PartialEq)]
enum Theme {
    MatrixInspired,
    EverythingBlue,
    VintageTerminal,
    Modern,
}

struct TerminalEntryData {
    req: String,
    resp: Element
}

struct ThemeStruct {
    theme: Theme,
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
            ("/about/secrets/shhh", "There's an `exit` command. Don't try that."),
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


    let date_string = format!("{}", Local::now().format("%a, %m/%d/%Y, %l:%M:%S%p UTC%Z"));
    let mut theme = use_signal(|| ThemeStruct {theme: Theme::Modern, });
    rsx! {
        link { rel: "stylesheet", href:  match theme.read().theme {
            Theme::MatrixInspired => { "matrix-inspired.css" },
            Theme::EverythingBlue => { "blue-everywhere.css" },
            Theme::VintageTerminal => { "vintage-terminal.css" },
            Theme::Modern => { "modern.css" }
        }},
        if theme.read().theme != Theme::Modern {
            link { rel: "stylesheet", href: "main.css" }
        }
        div { class:"root-container", 
            div { class:"container", padding: "0.5rem", position: "relative", font_size: "1.5rem",
                p { "Welcome to Raghu's Terminal! (v0.1.0)" },
                p { "{date_string}" },
                div { class:"github-container",
                    span { "Check out the source at " },
                    a { href: "https://github.com/orangatun/website-rs", "https://github.com/orangatun/website-rs" },
                },
                p { margin: "1em 0.5em 0.5em 0.5em",
                    "Use 'help' command to list all available commands" 
                },
            }, 
            div { class:"container",
                ul {
                    class: "entries-list",
                    for id in start_from()..ending() {
                        TerminalEntry { key: "{id}", id, entries, current_path, theme}
                    }
                },
                TerminalActiveEntry { entries , start_from, ending , current_path, theme},
            }
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
        if current_path().len()==1 {
            return build_element("Already at root directory. Cannot go higher.");
        }
        for i in 1..current_path().len() {
            let key = i as u8;
            current_path.write().remove(&key);
        }
        return build_element("Changed directory to root.")
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
                    let filename: String = dir[0..dir.len()-1].to_string();
                    let key: u8 = current_path().len() as u8;
                    if contents.contains(&dir.as_str()) {
                        current_path.write().insert(key, dir);
                        return build_element(format!("Path changed to {}", build_abs_path(current_path)).as_str());
                    } else if contents.contains(&filename.as_str()) {
                        return build_element(format!("`{}` is not a directory. It's a file.", filename).as_str());
                    } else {
                        return resolve_error(Errors::DirectoryNotFound);
                    }
                }
            }
        }
    }
}

fn resolve_error(err: Errors) -> Element {
    rsx! {
        p { class: "resp-container",
            match err {
                Errors::CommandNotFound => "Command not found. Use 'help' command to see list of commands.",
                Errors::ExtraParametersPassed => "Extra parameters found in the command.",
                Errors::PathNotFound => "Path not found.",
                Errors::DirectoryNotFound => "Directory not found. Please check spelling and case.",
                Errors::FileNotFound => "File not found in directory. Please check spelling and case.",
                Errors::FileNotDirectory => "This is a file, not a directory. Directories end with a '/'"
            }
        }
    }
}

fn resolve_exit() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div { id: "exit-container",
            p { "There's no exiting this terminal." },
            p { "The only way out is turning off the internet." }
            p { class: "whisper-text", "Or, you could close this tab." }
        }
    }
}

fn help_response() -> Element {

    static HELP_CMD : [(&str, &str); 8] = [
        ("ls", "Lists files and directories inside current directory."),
        ("cd", "Changes directory to the go up one level (parent), or down one level (child)."),
        ("pwd", "Lists the full path of current working directory."),
        ("cat", "Used to print the contents of a file."),
        ("dog", "Used to print the contents of a file."),
        ("help", "Displays this list of commands available."),
        ("clear", "Clears the terminal."),
        ("theme", "Change the theme of the terminal.")
    ];

    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        table {
            tr {
                th { class: "table-header",
                    "command"
                }, 
                th { class: "table-header",
                "description"
                }
            },
            for (cmd, desc) in HELP_CMD {
                tr {
                    td { class: "table-cmd",
                        "{cmd}"
                    }, 
                    td {
                        "{desc}"
                    }
                }
            }
        }
    }
}

fn theme_help() -> Element {
    static THEME_HELP : [(&str, &str); 4] = [
        ("terminal", "A vintage terminal feel, with a green background."),
        ("blue", "Everything is a shade of blue."),
        ("matrix", "Black background; green text."),
        ("modern", "A modern design.")
    ];

    rsx! {
        div {
            p { margin: "1em 1em 1em 0",
                "There are four options for themes:"
            },
            table {
                tr {
                    th { class: "table-header",
                        "option"
                    }, 
                    th { class: "table-header",
                        "description"
                    }
                },
                for (option, desc) in THEME_HELP {
                    tr {
                        td { class: "table-cmd",
                            "{option}"
                        }, 
                        td {
                            "{desc}"
                        }
                    }
                }
            },
            p { margin: "1em 1em 1em 0",
                "Usage: theme [option]"
            }
        }
    }
}

fn resolve_theme(words: Vec::<String>, mut theme: Signal<ThemeStruct>) -> Element {
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
        // theme.write().theme = Theme::MatrixInspired;
        
        let curr_theme = match theme.read().theme {
            Theme::VintageTerminal => "terminal",
            Theme::EverythingBlue => "blue",
            Theme::MatrixInspired => "matrix",
            Theme::Modern => "modern"
        };
        return build_element(format!("Theme is set to `{}`. To learn more, type \"theme help\"", curr_theme).as_str());

    } else {
        let mut theme_changed = false;
        match word.trim() {
            "terminal" => if(theme.read().theme != Theme::VintageTerminal) {
                theme.write().theme = Theme::VintageTerminal;
                theme_changed = true;
            },
            "blue" => if(theme.read().theme != Theme::EverythingBlue) {
                theme.write().theme = Theme::EverythingBlue;
                theme_changed = true;
            },
            "matrix" => if(theme.read().theme != Theme::MatrixInspired) {
                theme.write().theme = Theme::MatrixInspired;
                theme_changed = true;
            },
            "modern" => if(theme.read().theme != Theme::Modern) {
                theme.write().theme = Theme::Modern;
                theme_changed = true;
            },
            "help" => return theme_help(),
            _ => {
                return rsx!(
                    div {
                        p { margin: "1em 1em 1em 0",
                            "Theme `{word.trim()}` not found."
                        },
                        {theme_help()}
                    }
                );
            }
        }
        if theme_changed {
            return build_element(format!("Theme set to `{}`.", word.trim()).as_str());
        } else {
            return build_element(format!("Theme is already set to `{}`.", word.trim()).as_str());
        }
    }
}


fn generate_response(req: String, current_path: Signal<HashMap::<u8, String>>, theme: Signal<ThemeStruct>) -> Element {
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
            },
            "theme" => {
                return resolve_theme(words, theme);
            },
            _ => {
                return resolve_error(Errors::CommandNotFound);
            }
        }
    }
}

#[component]
fn TerminalActiveEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, mut start_from: Signal<u32>, mut ending: Signal<u32>, mut current_path: Signal<HashMap::<u8, String>>, mut theme: Signal<ThemeStruct> ) -> Element {
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
                    resp: generate_response(draft.to_string(), current_path, theme),
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
        div { class: "entry-container",
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
}

#[component]
fn TerminalEntry(mut entries: Signal<HashMap<u32, TerminalEntryData>>, id: u32, current_path: Signal<HashMap::<u8, String>>, theme: Signal<ThemeStruct>) -> Element {
    let req = entries.read().get(&id).unwrap().req.clone();
    let resp = entries.read().get(&id).unwrap().resp.clone();

    rsx! {
        link { rel: "stylesheet", href: "terminal_prompt.css" },
        div { class: "entry-container",
            span {
                class:"prompt", ">"
            },
            p { 
                class:"req-container",
                "{req}"
            },
            {resp}
        }
    }
}