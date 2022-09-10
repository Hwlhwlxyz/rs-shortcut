use std::collections::HashMap;
use regex::Regex;
use tinytemplate::TinyTemplate;

pub mod execution;

fn check_variables(cmd: &str) -> Vec<String> {
    let re = Regex::new(r"\{((?:.|\r?\n)+?)\}").unwrap();
    let mut variables_vec: Vec<String> = Vec::new();
    for cap in re.captures_iter(cmd) {
        // println!("{:?} - [{}]", &cap[0], &cap[1]);
        variables_vec.push(cap[1].trim().to_string())
    }
    variables_vec
}

// apply variables (hashmap<Variable, InputValue>) to the template
fn apply_template(template: String, var_map:HashMap<String, String>) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("hello", template.as_str()).unwrap();
    let rendered = tt.render("hello", &var_map).unwrap();
    println!("{}", rendered);
    rendered
}