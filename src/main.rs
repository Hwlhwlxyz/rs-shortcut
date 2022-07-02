use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::*;
use std::fs;
use std::hash::Hash;
use std::string::String;

use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::net::TcpStream;
use inquire::error::{InquireError, InquireResult};
use inquire::{Select, Text};
use toml::{from_str, Value};
use toml::value::Table;
use ssh2::Session;
use std::process::{ChildStdout, Command, Output, Stdio};
use std::str::from_utf8;
use std::io::Write;
use inquire::formatter::OptionFormatter;
use tinytemplate::TinyTemplate;

use encoding::all::GBK;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use serde::de::Unexpected::Str;
use regex::Regex;


#[derive(Deserialize, Debug)]
struct Item {
    foo: u64,
    bar: u64,
}

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

fn print_gbk(s: &[u8]) {
    let decoded_string = GBK.decode(s, DecoderTrap::Strict).unwrap();
    print!("{}", decoded_string);
}

fn print_utf8(s: &[u8]) {
    let decoded_string = String::from_utf8_lossy(s);
    print!("{}", decoded_string);
}

fn display_buffer(buffer:&[u8], coding:String) {
    match coding.as_str() {
        "gbk" => {print_gbk(buffer)},
        // "utf8" => {print_utf8(buffer)},
        _ => {print_utf8(buffer)}
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
    let vars_in_cmd = check_variables(snpt.command.as_str());
    let vars_map;
    if vars_in_cmd.len()>0 {
        vars_map = input_variables_then_to_map(vars_in_cmd);
        println!("vars_map: {:?}", vars_map);
        run_cmd = apply_template(snpt.command, vars_map).to_string();
    }

    println!("execute_command: {} {:?}", shell, run_cmd);
    let output = match shell.as_str() {
        "cmd" => {
            Command::new(shell).arg("/C")
                .args(run_cmd.split(' '))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap()
        },
        _ => {
            Command::new(shell)
                .args(run_cmd.split(' '))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap()
        }
    };

    let mut reader = BufReader::new(output);

    loop {
        let buffer = reader.fill_buf().unwrap();
        let consumed = {
            display_buffer(buffer, "gbk".to_string());
            buffer.len()
        };

        reader.consume(consumed);
        if consumed==0 { break; }
    }

    println!("fn finish")
}

fn check_variables(cmd: &str) -> Vec<String> {
    let re = Regex::new(r"\{((?:.|\r?\n)+?)\}").unwrap();
    println!("check_variables");
    let mut variables_vec: Vec<String> = Vec::new();
    for cap in re.captures_iter(cmd) {
        println!("{:?} - [{}]", &cap[0], &cap[1]);
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
            Err(_) => println!("An error happened when asking for your name, try again later."),
        }
    }
    var_map
}

fn main() {
    println!("Hello, world!");

    let filename = "./config.toml";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let items_string: &str = contents.as_str();
    let parsed_values = items_string.parse::<Value>().unwrap();
    println!("{}", parsed_values);
    parsed_values.as_table();

    println!("{}",parsed_values["config"]);
    println!("parsed_values[snippets]");
    for x in parsed_values["snippets"].as_array() {
        println!("{:?}", x.as_slice());
    }
    let snippet_items: Configuration = from_str(&*contents).unwrap();
    println!("{:?}", snippet_items.snippets);
    println!("{:?}", snippet_items.config);

    let snippets_vec = snippet_items.snippets;
    println!("snippets_vec");
    for s in &snippets_vec {
        println!("{}", s.command);
        let variables_vec = check_variables(s.command.as_str());
        println!("{:?}, length:{}", variables_vec, variables_vec.len());
    }

    let shell_name = snippet_items.config.get("shell").unwrap();
    let formatter: OptionFormatter<Snippet> = &|i| format!("Snippet {}: '{}'", i.index + 1, i);
    let ans: InquireResult<Snippet> = Select::new("Select configuration:", snippets_vec)
        .with_formatter(formatter)
        .with_help_message("help message")
        .prompt();

    // println!("{:?}", ans);

    match ans {
        Ok(snippet) => {
            let output = execute_command(shell_name, snippet);
            println!("result output: {:?}", output);
        },
        Err(err) => println!("error:{:?}", err)
    }

    println!("finished");

}