//! Rukinia - A testing and analysis tool
//!
//! This is the main entry point for the Rukinia application. It handles command-line arguments,
//! configuration, and execution of the main functionality.
//!
//! # Features
//! - Interactive shell mode for manual command execution
//! - Configurable test execution
//! - Multiple output formats for test results (CSV, Text, JUnit)
//! - Performance timing
//!
//! # Usage
//! See the `print_help()` function for command-line options or run with `--help`

mod core;
mod tasks;

use core::save_test_result::ResultFormat;
use std::env;
use std::error::Error;
use std::time::Instant;

use crate::core::interactive_shell::interactive_shell;
use crate::core::configuration::rukinia_use_settings;
use crate::core::run_tasks::rukinia_run_analysis;

use crate::core::save_test_result::FormatOutput;

/// Prints help information about command-line options
fn print_help() {
    println!("Usage: Rukinia [OPTIONS]");
    println!("\nOptions:");
    println!("  help      Show this help message and exit");
    println!("  config    Specify a configuration file (default: config)");
    println!("  shell     Shell to manually enter rukinia commands");
    println!("  save-csv  Save test result in a CSV file");
    println!("  custom-path-csv  File path of the output file for CSV");
}

/// Main entry point for Rukinia application
///
/// # Returns
/// Returns `Result<(), Box<dyn Error>>` indicating success or failure of the application
///
/// # Examples
/// Basic usage:
/// ```
/// // Typically run from command line:
/// // rukinia --help
/// ```
///
/// Running tests and saving CSV results:
/// ```
/// // rukinia --save-csv --custom-path-csv my_results.csv
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut result_format: Option<ResultFormat> = None;

    if args.contains(&"help".to_string()) {
        print_help();
        return Ok(());
    }

    if args.contains(&"shell".to_string()) {
        interactive_shell();
        return Ok(());
    }

    let runtime = rukinia_use_settings();

    let custom_path = args
        .iter()
        .position(|arg| arg == "custom-path")
        .and_then(|index| args.get(index + 1).cloned())
        .unwrap_or_else(|| "rukinia".to_string());
    
    if args.contains(&"save-csv".to_string()) {
        result_format = Some(ResultFormat {
            format: FormatOutput::Csv,
            path: custom_path,
        });
    } else if args.contains(&"save-text".to_string()) {
        result_format = Some(ResultFormat {
            format: FormatOutput::TextFile,
            path: custom_path,
        });
    } else if args.contains(&"save-junit".to_string()) {
        result_format = Some(ResultFormat {
            format: FormatOutput::JUnit,
            path: custom_path,
        });
    } 

    match runtime {
        Ok(run) => run.block_on(rukinia_run_analysis(result_format)),
        Err(e) => {
            eprintln!("Failed to create runtime: {}", e);
            return Ok(());
        }
    };

    let elapsed_time = start_time.elapsed();
    println!("Time spent: {:.3}", elapsed_time.as_secs_f64());

    Ok(())
}
