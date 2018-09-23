use std::collections::HashMap;
use std::env;
use std::process::{exit, Command, Output, Stdio};
use std::str;

fn main() {
    let input_arguments: Vec<String> = env::args().collect();

    let interpreters = ["php", "php70", "php71", "php72"];
    let mut interpreter = String::new();
    let mut binary = String::new();

    let mut aliases = HashMap::new();
    aliases.insert(string("m1"), string("n98-magerun"));
    aliases.insert(string("m2"), string("n98-magerun2"));
    aliases.insert(string("cc"), string("composer"));

    // Start from index one, since first element is the executed binary, which we don't need right now.
    for i in 1..input_arguments.len() {
        let arg: &str = &input_arguments[i];

        if i == 1 {
            if interpreters.contains(&arg) {
                interpreter = arg.to_string();
            } else  {
                binary = arg.to_string();
            }
        }

        if i == 2 && !interpreter.is_empty() {
            binary = arg.to_string();
            break;
        }
    }

    if binary.is_empty() {
        println!("No binary given.");
        exit(1);
    }

    if aliases.contains_key(&binary) {
        binary = aliases.get(&binary).unwrap().to_string();
    }

    let mut executable = binary;
    let mut command_arguments_index = 2;
    let mut command_arguments = Vec::new();

    if !interpreter.is_empty() {
        let output = Command::new("which")
                .arg(executable)
                .output()
                .expect("Execution of 'which' failed.");

        command_arguments.push(output_to_string(output));

        executable = interpreter;
        command_arguments_index += 1;
    }

    for i in command_arguments_index..input_arguments.len() {
        &command_arguments.push(input_arguments[i].to_owned());
    }

    let status = Command::new(executable)
              .args(command_arguments)
              .stdin(Stdio::inherit())
              .stdout(Stdio::inherit())
              .stderr(Stdio::inherit())
              .status()
              .expect("Failed to execute command");

    exit(status.code().unwrap());
}

/**
 * Create String from str.
 */
fn string(value: &str) -> String {
    return value.to_owned();
}

/**
 * Get string from Output type.
 *
 * @TODO: What about stderr?
 */
fn output_to_string(output: Output) -> String {
    return String::from_utf8(output.stdout).unwrap().trim().to_string();
}
