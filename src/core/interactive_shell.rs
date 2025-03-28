use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::core::run_tasks::rukinia_execute_single_test;
use tokio::runtime::Runtime;

const HISTORY_FILE: &str = ".rukinia_history";

pub fn load_history() -> Vec<String> {
    if Path::new(HISTORY_FILE).exists() {
        match fs::read_to_string(HISTORY_FILE) {
            Ok(contents) => contents.lines().map(|s| s.to_string()).collect(),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

pub fn save_to_history(command: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)
        .expect("Failed to open history file");

    writeln!(file, "{}", command).expect("Failed to write to history file");
}

pub fn interactive_shell() {
    let rt = Runtime::new().unwrap();
    let mut history = load_history();

    println!("Welcome to the Rukinia Shell!");
    println!("Type your commands below. Type `quit()` or `exit` to exit.");
    println!("Use `history` to view past commands, `!N` to run a previous command.");

    loop {
        print!("rukinia> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Try again.");
            continue;
        }

        let command = input.trim().to_string();
        if command.is_empty() {
            continue;
        }

        if command == "quit()" || command == "exit" {
            println!("Exiting Rukinia Shell...");
            break;
        } else if command == "history" {
            for (i, cmd) in history.iter().enumerate() {
                println!("{}: {}", i + 1, cmd);
            }
            continue;
        } else if command.starts_with('!') {
            if let Ok(index) = command[1..].parse::<usize>() {
                if index > 0 && index <= history.len() {
                    let previous_command = &history[index - 1];
                    println!("Running: {}", previous_command);
                    rt.block_on(async {
                        match rukinia_execute_single_test(&previous_command).await {
                            Ok(result) => result.display_result(),
                            Err(err) => err.display_result(),
                        };
                    });
                } else {
                    println!("Invalid history index.");
                }
            } else {
                println!("Invalid history command. Use `!N` to rerun a command.");
            }
            continue;
        }

        save_to_history(&command);
        history.push(command.clone());

        rt.block_on(async {
            match rukinia_execute_single_test(&command).await {
                Ok(result) => result.display_result(),
                Err(err) => err.display_result(),
            };
        });
    }
}
