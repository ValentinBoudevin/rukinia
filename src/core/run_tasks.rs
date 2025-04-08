/// Rukinia Test Execution and Analysis Module
///
/// This module provides functionality for executing and analyzing tests within
/// the Rukinia framework. It handles reading test files, executing tests, and
/// formatting the results in various output formats.
///
/// # Overview
/// The primary function of this module is to process a series of test expressions,
/// evaluate them, and output the results in a specified format. It involves
/// tasks such as reading input files, processing logical operators, and interacting
/// with task execution systems.
///
/// # Example Usage
/// ```rust,ignore
/// // Run analysis with an optional output format (e.g., CSV, JUnit)
/// rukinia_run_analysis(None).await;
/// ```
///
/// # Functions
/// - `rukinia_run_analysis`: Reads a test file and executes all tests within it.
/// - `rukinia_execute_all_tests`: Iterates over the test expressions in a buffer and executes each one.
/// - `rukinia_execute_single_test`: Processes a single test expression, handling logical operators and parentheses.
/// - `evaluate_logical_expression`: Evaluates an expression with logical operators like AND/OR.
/// - `convert_config_line_to_vector_string`: Converts a configuration line into a vector of strings.
/// - `find_outermost_parentheses`: Finds the outermost parentheses in an expression.
/// - `read_expression_task`: Reads a task expression and returns the corresponding result and remaining parts.
/// - `create_task`: Creates a task from the parts of an expression.
/// - `execute_task`: Executes a task with the given arguments and syntax.
/// - `write_result`: Writes the result of a test to a file in a specified format.
/// - `write_result_error`: Handles writing error results to a specified format.
use regex::Regex;
use std::fs::File;
use std::str::FromStr;

use crate::core::syntax::SyntaxForTrait;

use crate::tasks::bool::rukinia_false::RukiniaFalse;
use crate::tasks::bool::rukinia_true::RukiniaTrue;
use crate::tasks::task::*;

use crate::core::configuration::rukinia_open_test_file;
use crate::core::configuration::rukinia_read_test_file;

use super::rukinia_result::RukiniaResultType;
use super::save_test_result::FormatOutput;
use crate::core::save_test_result::ResultFormat;

use crate::core::operator::LogicalOperator;
use crate::core::syntax::Syntax;
use once_cell::sync::Lazy;
use std::error::Error;

use crate::core::rukinia_result::RukiniaError;
use crate::core::rukinia_result::RukiniaResultEntry;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#""([^"]*)"|(\S+)"#).unwrap());

/// Reads a test file and executes all tests within it.
///
/// This function opens a test file, reads its contents, and processes the test
/// expressions. It also handles formatting the output of each test based on
/// the specified result format (e.g., CSV, Text, JUnit).
///
/// # Arguments
/// * `result_format` - Optional argument that specifies the format for the result output.
///
/// # Example
/// ```rust,ignore
/// rukinia_run_analysis(None).await;
/// ```
pub async fn rukinia_run_analysis(result_format: Option<ResultFormat>) {
    let mut file: Option<File> = None;
    let mut contents = String::new();

    rukinia_open_test_file(&mut file);

    let mut file = match file {
        Some(f) => f,
        None => return,
    };

    rukinia_read_test_file(&mut file, &mut contents);
    drop(file);

    rukinia_execute_all_tests(&contents, result_format).await;
}

/// Executes all tests found in the input buffer.
///
/// This function iterates through each line in the input buffer, executes
/// each test expression, and displays the results. It also handles writing
/// the results to the specified output format.
///
/// # Arguments
/// * `buffer` - The string containing multiple test expressions, each on a new line.
/// * `output_format` - Optional result format specifying how the output should be written (e.g., CSV, JUnit).
///
/// # Example
/// ```rust,ignore
/// rukinia_execute_all_tests("test1\n test2", None).await;
/// ```
///
pub async fn rukinia_execute_all_tests(buffer: &str, output_format: Option<ResultFormat>) {
    for line in buffer.lines() {
        match rukinia_execute_single_test(line).await {
            Ok(rukinia_entry) => {
                rukinia_entry.display_result();
                if let Some(ref valid_format) = output_format {
                    if let Err(e) = write_result(&rukinia_entry, valid_format).await {
                        RukiniaError::new(
                            format!("Write {}", valid_format.format),
                            "Failed to write output".to_string(),
                            e.to_string(),
                        )
                        .display_result();
                    }
                }
            }
            Err(rukinia_error) => {
                rukinia_error.display_result();
                if let Some(ref valid_format) = output_format {
                    if let Err(e) = write_result_error(&rukinia_error, valid_format).await {
                        RukiniaError::new(
                            format!("Write {}", valid_format.format),
                            "Failed to write error output".to_string(),
                            e.to_string(),
                        )
                        .display_result();
                    }
                }
            }
        };
    }
}

/// Executes a single test expression and handles the evaluation of logical operators.
///
/// This function processes a single test expression, checking for parentheses and
/// logical operators such as AND/OR. It recursively evaluates sub-expressions
/// and combines results accordingly.
///
/// # Arguments
/// * `expression` - The test expression string that needs to be evaluated.
///
/// # Example
/// ```rust,ignore
/// let result = rukinia_execute_single_test("test1 && test2").await;
/// ```
///
/// # Errors
/// Returns a `RukiniaError` if there's an issue with parsing or evaluating the expression.
///
pub async fn rukinia_execute_single_test(
    expression: &str,
) -> Result<RukiniaResultEntry, RukiniaError> {
    if let Some((start, end)) = find_outermost_parentheses(expression) {
        let content = &expression[start + 1..end];
        let result = Box::pin(rukinia_execute_single_test(content)).await?;

        let result_bool_str = match result.result_type {
            RukiniaResultType::TestSuccess => RukiniaTrue::get_rukinia_command(),
            RukiniaResultType::TestFail => RukiniaFalse::get_rukinia_command(),
        };

        let new_expression = format!(
            "{}{}{}",
            &expression[..start],
            result_bool_str,
            &expression[end + 1..]
        );

        let mut final_result = Box::pin(rukinia_execute_single_test(&new_expression)).await?;

        if find_outermost_parentheses(new_expression.as_str()).is_none() {
            let new_expression_vec = convert_config_line_to_vector_string(new_expression.as_str());
            if let Some(syntax) = Syntax::find_syntax_as(new_expression_vec).await {
                final_result.label = syntax.get_argument().unwrap();
            } else {
                final_result.label = format!(
                    "{}({}){}",
                    &expression[..start],
                    result.label,
                    &expression[end + 1..],
                );
            }
        } else {
            let new_expression_vec = convert_config_line_to_vector_string(new_expression.as_str());
            if let Some(syntax) = Syntax::find_syntax_as(new_expression_vec).await {
                final_result.label = syntax.get_argument().unwrap();
            } else {
                final_result.label = format!(
                    "{}({}){}",
                    &final_result.label[..start],
                    result.label,
                    &final_result.label[start + result_bool_str.len()..],
                );
            };
        }

        return Ok(final_result);
    }

    let formatted_parts = convert_config_line_to_vector_string(expression);
    evaluate_logical_expression(formatted_parts).await
}

/// Converts a configuration line (string) into a vector of strings based on regex matches.
///
/// # Arguments
/// * `buffer` - The string to be converted into a vector.
///
/// # Returns
/// A vector of strings extracted from the buffer using regex.
fn convert_config_line_to_vector_string(buffer: &str) -> Vec<String> {
    RE.captures_iter(buffer)
        .filter_map(|cap| {
            cap.iter()
                .skip(1)
                .flatten()
                .next()
                .map(|m| m.as_str().to_string())
        })
        .collect()
}

/// Evaluates logical expressions with operators (AND, OR).
///
/// This function evaluates expressions containing logical operators like AND or OR.
/// It processes each operator and recursively evaluates sub-expressions.
///
/// # Arguments
/// * `expression_parts` - A vector of strings representing parts of a logical expression.
///
/// # Returns
/// The final result of the evaluated expression.
async fn evaluate_logical_expression(
    expression_parts: Vec<String>,
) -> Result<RukiniaResultEntry, RukiniaError> {
    let (result, mut remaining_parts) = read_expression_task(expression_parts.clone()).await?;
    let mut final_result = result.clone();

    while !remaining_parts.is_empty() {
        let operator = &remaining_parts[0];

        let logical_operator = match operator.parse::<LogicalOperator>() {
            Ok(op) => op,
            Err(_) => {
                return Err(RukiniaError::new(
                    "Invalid operator".to_string(),
                    "Failed to parse logical operator".to_string(),
                    format!("Invalid operator: {}", operator),
                ));
            }
        };

        let (new_task_result, new_remaining) =
            read_expression_task(remaining_parts[1..].to_vec()).await?;

        match logical_operator {
            LogicalOperator::And => {
                if final_result.result_type == RukiniaResultType::TestSuccess {
                    final_result.result_type = new_task_result.result_type;
                }
                final_result.label.push_str(" and ");
                final_result.label.push_str(new_task_result.label.as_str());
            }
            LogicalOperator::Or => {
                if final_result.result_type == RukiniaResultType::TestFail {
                    final_result.result_type = new_task_result.result_type;
                }
                final_result.label.push_str(" or ");
                final_result.label.push_str(new_task_result.label.as_str());
            }
        }

        remaining_parts = new_remaining;
    }
    Ok(final_result)
}

/// Finds the outermost parentheses in an expression.
///
/// # Arguments
/// * `expression` - The string containing the expression.
///
/// # Returns
/// The indices of the outermost parentheses if found, or `None` if not found.
fn find_outermost_parentheses(expression: &str) -> Option<(usize, usize)> {
    let mut start_index = None;
    let mut balance = 0;

    for (i, ch) in expression.chars().enumerate() {
        if ch == '(' {
            if start_index.is_none() {
                start_index = Some(i);
            }
            balance += 1;
        } else if ch == ')' {
            balance -= 1;
            if balance == 0 {
                if let Some(start) = start_index {
                    return Some((start, i));
                }
            }
        }
    }
    None
}

/// Reads a task expression and returns the corresponding result and remaining parts.
///
/// This function processes the parts of the expression and determines if it's a valid task
/// or if more syntax extraction is needed.
///
/// # Arguments
/// * `parts` - The vector of strings representing parts of the expression.
///
/// # Returns
/// A result containing a `RukiniaResultEntry` and the remaining parts of the expression.
async fn read_expression_task(
    parts: Vec<String>,
) -> Result<(RukiniaResultEntry, Vec<String>), RukiniaError> {
    let mut syntax = SyntaxForTrait::Arguments(Vec::new());
    let mut remaining_parts = parts.clone();
    let mut bool_syntax_extracted = false;

    while !remaining_parts.is_empty() {
        let word = &remaining_parts[0];
        match RukiniaAllTasks::from_str(word.as_str()) {
            Ok(_) => {
                let (task_result, arguments, remaining) = create_task(remaining_parts.clone())?;
                let rukinia_valid = execute_task(task_result, arguments.clone(), syntax).await?;
                return Ok((rukinia_valid, remaining));
            }
            Err(_) => {
                if bool_syntax_extracted {
                    return Err(RukiniaError::new(
                        "Invalid task".to_string(),
                        "Failed to parse rukinia task".to_string(),
                        format!("Invalid task: {}", word),
                    ));
                }
                let (new_syntax_trait, new_remaining) =
                    SyntaxForTrait::extract_syntax(remaining_parts).await?;
                syntax = new_syntax_trait;
                remaining_parts = new_remaining;
                bool_syntax_extracted = true;
            }
        }
    }
    Err(RukiniaError::new(
        "No task found in the test".to_string(),
        "Failed to parse rukinia task".to_string(),
        format!("No task in test: {}", parts.join(" ")),
    ))
}

/// Creates a task from a set of parts in the expression.
///
/// This function extracts the task name, its arguments, and any remaining parts of the
/// expression that might represent logical operators or additional syntax.
///
/// # Arguments
/// * `parts` - A vector of strings representing the parts of the expression, where the first
///   part is the task name, followed by its arguments and potential operators.
///
/// # Returns
/// A result containing:
/// - The task (`RukiniaAllTasks`) that corresponds to the task name found in the expression.
/// - A vector of strings representing the arguments passed to the task.
/// - A vector of strings containing the remaining parts (typically operators and additional expressions).
///
/// # Errors
/// Returns a `RukiniaError` if the task name is invalid or cannot be parsed.
fn create_task(
    parts: Vec<String>,
) -> Result<(RukiniaAllTasks, Vec<String>, Vec<String>), RukiniaError> {
    let task = match RukiniaAllTasks::from_str(&parts[0]) {
        Ok(task) => task,
        Err(_) => {
            return Err(RukiniaError::new(
                "Invalid task name".to_string(),
                "Failed to parse rukinia task".to_string(),
                format!("Invalid task: {}", parts[0]),
            ));
        }
    };

    // Collect arguments until we hit a logical operator or end of parts
    let mut arguments = Vec::new();
    let mut remaining_parts = Vec::new();
    let mut found_operator = false;

    for part in parts.iter().skip(1) {
        if LogicalOperator::from_str(part).is_ok() {
            found_operator = true;
        }

        if found_operator {
            remaining_parts.push(part.clone());
        } else {
            arguments.push(part.clone());
        }
    }

    Ok((task, arguments, remaining_parts))
}

/// Executes the specified task with the given arguments and syntax.
///
/// This function calls the appropriate execution method for the given task, passing in the arguments
/// and syntax associated with it. It handles the interaction with the task execution system and returns
/// the result of the task.
///
/// # Arguments
/// * `rukinia_task` - The task to be executed, represented by the `RukiniaAllTasks` enum.
/// * `arguments` - The arguments to be passed to the task during execution.
/// * `syntax` - The syntax rules that should be applied to the task.
///
/// # Returns
/// A result containing a `RukiniaResultEntry` that holds the result of the task execution, which
/// can be either successful or an error result.
///
/// # Errors
/// Returns a `RukiniaError` if the task execution fails, such as if the task is not recognized
/// or the arguments are incorrect.
async fn execute_task(
    rukinia_task: RukiniaAllTasks,
    arguments: Vec<String>,
    syntax: SyntaxForTrait,
) -> Result<RukiniaResultEntry, RukiniaError> {
    rukinia_task.execute(arguments, syntax).await
}

/// Writes the result of a task to a specified output format (CSV, Text, JUnit).
///
/// # Arguments
/// * `task_result` - The result of a test task.
/// * `format` - The format in which the result should be written (e.g., CSV, JUnit).
async fn write_result(
    task_result: &RukiniaResultEntry,
    format: &ResultFormat,
) -> Result<(), Box<dyn Error>> {
    match format.format {
        FormatOutput::Csv => task_result.write_csv(&format.path).await?,
        FormatOutput::TextFile => task_result.write_text(&format.path).await?,
        FormatOutput::JUnit => task_result.write_junit(&format.path).await?,
    }
    Ok(())
}

/// Handles writing error results to the specified format (CSV, Text, JUnit).
///
/// # Arguments
/// * `error` - The error result to be written.
/// * `format` - The format in which the error result should be written.
async fn write_result_error(
    error: &RukiniaError,
    format: &ResultFormat,
) -> Result<(), Box<dyn Error>> {
    match format.format {
        FormatOutput::Csv => error.write_csv(&format.path).await?,
        FormatOutput::TextFile => error.write_text(&format.path).await?,
        FormatOutput::JUnit => error.write_junit(&format.path).await?,
    }
    Ok(())
}
