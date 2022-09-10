

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::*;
use std::fs;
use std::string::String;
use inquire::error::{InquireResult};
use inquire::{Select};
use toml::{from_str};
use inquire::formatter::OptionFormatter;



mod snippet;
pub use crate::snippet::Snippet as Snippet;
mod util;
pub use crate::util::execution as execution;

#[derive(Deserialize, Debug)]
struct Configuration {
    config: HashMap<String, String>,
    snippets: Vec<Snippet>
}

fn main() {
    let filename = "./config.toml";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let snippet_items: Configuration = from_str(&*contents).unwrap();
    let snippets_vec = snippet_items.snippets;
    let shell_name = snippet_items.config.get("shell").unwrap();
    let formatter: OptionFormatter<Snippet> = &|i| format!("Snippet {}: '{}'", i.index + 1, i);
    let ans: InquireResult<Snippet> = Select::new("Select a command:", snippets_vec)
        .with_formatter(formatter)
        .with_help_message("help message")
        .prompt();

    match ans {
        Ok(snippet) => {
            execution::execute_command(shell_name, snippet);
        },
        Err(err) => println!("error:{:?}", err)
    }
}