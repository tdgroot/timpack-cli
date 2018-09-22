use std::env;
use std::collections::HashMap;
use std::process::{exit, Command, Stdio};
use std::str;

fn main() {
    let input_arguments: Vec<String> = env::args().collect();

    let interpreters = ["php", "php70", "php71", "php72"];
    let mut interpreter: &str = "";
    let mut binary: &str = "";

    let mut aliases = HashMap::new();
    aliases.insert("m1", "n98-magerun");
    aliases.insert("m2", "n98-magerun2");
    aliases.insert("cc", "composer");

    // Start from index one, since first element is the executed binary, which we don't need right now.
    for i in 1..input_arguments.len() {
        let arg: &str = &input_arguments[i];

        if i == 1 {
            if interpreters.contains(&arg) {
                interpreter = &arg;
            } else  {
                binary = &arg;
            }
        }

        if i == 2 && interpreter != "" {
            binary = &arg;
            break;
        }
    }

    if aliases.contains_key(binary) {
        binary = aliases.get(binary).unwrap();
    }

    let mut executable = binary;
    let mut command_arguments_index = 2;
    let mut command_arguments = Vec::new();

    if interpreter != "" {
        let output = Command::new("which")
                .arg(binary)
                .output()
                .expect("Execution of 'which' failed.");

        let binary_path = String::from_utf8(output.stdout).unwrap().trim().to_string();
        command_arguments.push(binary_path);

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
