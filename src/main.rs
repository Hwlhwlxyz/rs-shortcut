use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::*;
use std::fs;
use std::string::String;
use inquire::error::{InquireResult};
use inquire::{Select, Text};
use toml::{from_str};
use std::process::{Command};
use inquire::formatter::OptionFormatter;
use tinytemplate::TinyTemplate;
use regex::Regex;



#[derive(Deserialize, Debug)]
struct Configuration {
    config: HashMap<String, String>,
    snippets: Vec<Snippet>
}

#[derive(Deserialize, Debug)]
struct Snippet {
    description: String,
    command: String,
}

impl std::fmt::Display for Snippet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.command, self.description)
    }
}

fn apply_template(template: String, var_map:HashMap<String, String>) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("hello", template.as_str()).unwrap();

    let rendered = tt.render("hello", &var_map).unwrap();
    println!("{}", rendered);
    rendered
}

// rust Continuously process child process' outputs
fn execute_command(shell: &String, snpt: Snippet) {
    let mut run_cmd = snpt.command.to_string();

    if run_cmd.starts_with("ssh ")  { // if use ssh to connect to other servers
        Command::new("ssh")
            .arg(&run_cmd[4..run_cmd.len()])
            .spawn().unwrap()
            .wait().unwrap();
    }
    else {
        let vars_in_cmd = check_variables(snpt.command.as_str());
        let vars_map;
        if vars_in_cmd.len()>0 {
            vars_map = input_variables_then_to_map(vars_in_cmd);
            run_cmd = apply_template(snpt.command, vars_map).to_string();
        }
        match shell.as_str() {
            "cmd" => {
                Command::new(shell).arg("/C")
                    .arg(&run_cmd)
                    .spawn().unwrap()
                    .wait().unwrap();
            },
            _ => {
                Command::new(shell)
                    .arg(&run_cmd)
                    .spawn().unwrap()
                    .wait().unwrap();
            }
        };
    }
    // println!("fn execute_command finish")
}

fn check_variables(cmd: &str) -> Vec<String> {
    let re = Regex::new(r"\{((?:.|\r?\n)+?)\}").unwrap();
    let mut variables_vec: Vec<String> = Vec::new();
    for cap in re.captures_iter(cmd) {
        // println!("{:?} - [{}]", &cap[0], &cap[1]);
        variables_vec.push(cap[1].trim().to_string())
    }
    variables_vec
}

fn input_variables_then_to_map(variables_vec: Vec<String>) -> HashMap<String, String> {
    let mut var_map = HashMap::new();
    for v in variables_vec {
        let replace_value = Text::new(v.as_str()).prompt();
        match replace_value {
            Ok(replace_value) => {
                println!("Input {}", replace_value);
                var_map.insert(v, replace_value);
            },
            Err(_) => println!("An error happened when inputting, try again later."),
        }
    }
    var_map
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
            execute_command(shell_name, snippet);
        },
        Err(err) => println!("error:{:?}", err)
    }
    // println!("finished");
}