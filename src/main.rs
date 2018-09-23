use std::collections::HashMap;
use std::env;
use std::process::{exit, Command, Output, Stdio};
use std::str;

fn main() {
    let input_arguments: Vec<String> = env::args().collect();

    let interpreters = ["php", "php70", "php71", "php72"];
    let mut interpreter = String::new();
    let mut binary = String::new();

    // Create a hashmap for the command aliases.
    let mut aliases = HashMap::new();
    aliases.insert(string("m1"), string("n98-magerun"));
    aliases.insert(string("m2"), string("n98-magerun2"));
    aliases.insert(string("cc"), string("composer"));

    // Determine if first argument is interpreter or binary.
    let arg_first = &input_arguments[1];
    if interpreters.contains(&arg_first.as_str()) {
        interpreter = string(&arg_first);
    } else {
        binary = string(&arg_first);
    }

    // If interpreter is specified, find out the binary that needs to be executed.
    if !interpreter.is_empty() && input_arguments.len() >= 3 {
        let arg_second = &input_arguments[2];
        binary = string(&arg_second);
    }

    // Fail fast, no binary to run.
    if binary.is_empty() {
        println!("No binary given.");
        exit(1);
    }

    // Resolve alias usage if necessary.
    if aliases.contains_key(&binary) {
        binary = aliases.get(&binary).unwrap().to_string();
    }

    let mut executable = binary;
    let mut command_arguments_index = 2;
    let mut command_arguments = Vec::new();

    // Prepare command execution if binary needs to be run with interpreter.
    if !interpreter.is_empty() {
        let output = Command::new("which")
                .arg(executable)
                .output()
                .expect("Execution of 'which' failed.");

        command_arguments.push(output_to_string(output));

        executable = interpreter;
        command_arguments_index += 1;
    }

    // Push all remaining stdin args to the new command arguments
    for i in command_arguments_index..input_arguments.len() {
        &command_arguments.push(input_arguments[i].to_owned());
    }

    // Run command and wait for status.
    let status = Command::new(executable)
              .args(command_arguments)
              .stdin(Stdio::inherit())
              .stdout(Stdio::inherit())
              .stderr(Stdio::inherit())
              .status()
              .expect("Failed to execute command");

    // Exit program with status code of command execution.
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
