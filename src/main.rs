use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::*;
use std::fs;
use std::hash::Hash;
use std::string::String;

use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::net::TcpStream;
use inquire::error::{InquireError, InquireResult};
use inquire::Select;
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

// rust Continuously process child process' outputs
fn execute_command(shell: &String, snpt: Snippet) {
    println!("execute_command: {} {:?}", shell, snpt);
    let output = match shell.as_str() {
        "cmd" => {
            Command::new(shell).arg("/C")
                .args(snpt.command.split(' '))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap()
        },
        _ => {
            Command::new(shell)
                .args(snpt.command.split(' '))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap()
        }
    };


    // .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

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

    // println!("finish")
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

    for x in parsed_values["snippets"].as_array() {
        println!("{:?}", x.as_slice());
    }
    let snippet_items: Configuration = from_str(&*contents).unwrap();
    println!("{:?}", snippet_items.snippets);
    println!("{:?}", snippet_items.config);

    let snippets_vec = snippet_items.snippets;
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
            println!("{:?}", output);
        },
        Err(err) => println!("error:{:?}", err)
    }

    // println!("finished");
    
    // match parsed_values_str {
    //     Some(x) => println!("Some: x={:?}", x),
    //     None => println!("None")
    // }
    // for s in parsed_values_str {
    //     println!("{:?}", s);
    // }


    // println!("{}",parsed_values["commands"]);

    // static TEMPLATE : &'static str = "Hello {name}!";
    // let mut tt = TinyTemplate::new();
    // tt.add_template("hello", TEMPLATE).expect("add template error");
    // let mut context = HashMap::new();
    // context.insert("name".to_string(), "Json".to_string());
    // let rendered = tt.render("hello", &context).unwrap();
    // println!("{}", rendered);




}