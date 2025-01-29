use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::*;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::string::String;
use inquire::error::{InquireResult};
use inquire::{Select};
use inquire::formatter::OptionFormatter;
use toml::from_str;


mod util;
mod models;

pub use crate::models::snippet::Snippet as Snippet;
pub use crate::util::execution as execution;

#[derive(Deserialize, Debug)]
struct Configuration {
    config: HashMap<String, String>,
    snippets: Vec<Snippet>,
}

fn find_config_path() -> PathBuf {
    let home_path = dirs::home_dir().unwrap();
    let exe_path = env::current_exe().unwrap();
    // let home_config_path = home_path.join("config.toml");
    // home_config_path.is_file()
    let path_list = [
        home_path.join("rs_shortcut.toml"),
        exe_path.join("./rs_shortcut.toml"),
        exe_path.join("config.toml"),
        Path::new("./rs_shortcut.toml").to_path_buf(),
        Path::new("./config.toml").to_path_buf(),
    ];
    for possible_path in path_list {
        // println!("{:?}", possible_path);
        if possible_path.is_file() {
            return possible_path;
        }
    }
    println!("please create config file ($HOME/rs_shortcut.toml, ./config.toml, ./rs_shortcut.toml)");
    panic!("config file not found!");
}

fn filter_snippets_by_tag(snippet_vec: Vec<Snippet>, tag: String) -> Vec<Snippet> {
    let filtered_snippet = snippet_vec
        .iter()
        .filter(|&snippet| {
            match &snippet.tag {
                None => false,
                Some(tag_vec) => {
                    let found_tag = tag_vec.iter().any(|s| *s == tag);
                    if found_tag {
                        true
                    } else {
                        false
                    }
                }
            }
        })
        .map(|s| s.clone())
        .collect();
    filtered_snippet
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?} {}", args, args.len());
    // let tag = &args[1];
    // println!("{:?}", tag);
    let tag_value: Option<String> = match args.len() {
        l => {
            if (l <= 1) { None } else { Some(args[1].clone()) }
        }
    };

    // let filename = "./config.toml";
    let filename = find_config_path();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let snippet_items: Configuration = from_str(&*contents).unwrap();
    let snippets_vec = match tag_value {
        None => {snippet_items.snippets}
        Some(tag_value) => {filter_snippets_by_tag(snippet_items.snippets, tag_value)}
    };
    let shell_name = snippet_items.config.get("shell").unwrap();
    let formatter: OptionFormatter<Snippet> = &|i| format!("Snippet {}: '{}'", i.index + 1, i);
    let ans: InquireResult<Snippet> = Select::new("Select a command:", snippets_vec)
        .with_formatter(formatter)
        .with_help_message("select a command")
        .prompt();

    let current_path = env::current_dir().unwrap();
    println!("The current directory is {}", current_path.display());
    assert!(env::set_current_dir(&current_path).is_ok());
    match ans {
        Ok(snippet) => {
            execution::execute_command(shell_name, snippet);
        }
        Err(err) => println!("error:{:?}", err)
    }
}