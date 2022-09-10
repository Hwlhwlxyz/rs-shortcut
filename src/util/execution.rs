use std::collections::HashMap;
use std::process::Command;
use inquire::Text;
use crate::Snippet;
use crate::util::{apply_template, check_variables};

pub fn execute_command(shell: &String, snpt: Snippet) {
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