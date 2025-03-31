//! Rukinia Command Execution Module
//!
//! This module defines `RukiniaCmd`, a struct that represents a command execution
//! process within the Rukinia framework.
//!
//! # Overview
//! `RukiniaCmd` executes external commands, captures their output, and applies syntax rules.
//! It implements the `RukiniaProcess` trait.
//!
//! # Errors
//! If the command execution fails, `RukiniaCmd` returns a `RukiniaError`.

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
use std::io;
use std::process::{Command, Output};

/// Represents a Rukinia command execution process.
///
/// This struct stores the command arguments, syntax rules, and execution result.
pub struct RukiniaCmd {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaCmd {
    /// Creates a new `RukiniaCmd` instance.
    ///
    /// # Arguments
    /// * `arguments` - A vector of strings representing the command and its arguments.
    /// * `syntax` - The syntax rules applied to the command execution.
    ///
    /// # Returns
    /// Returns `Ok(RukiniaCmd)` if the command executes successfully, otherwise returns a `RukiniaError`.
    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        if arguments.is_empty() {
            return Err(RukiniaError::new(
                "rukinia_cmd".to_string(),
                "Missing command argument".to_string(),
                "No command provided".to_string(),
            ));
        }

        let mut rukinia_cmd = RukiniaCmd {
            arguments: arguments.clone(),
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let command = arguments[0].clone();
        let command_args = &arguments[1..];

        let output: io::Result<Output> = Command::new(command).args(command_args).output();

        if output.is_ok() && output.unwrap().status.success() {
            rukinia_cmd.result.result_type = RukiniaResultType::TestSuccess;
        }

        rukinia_cmd.apply_syntax();
        Ok(rukinia_cmd)
    }

    /// Returns the name of the Rukinia command.
    fn get_rukinia_command() -> &'static str {
        "rukinia_cmd"
    }

    /// Retrieves the result of the command execution.
    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    /// Formats and returns a string representation of the command execution status.
    fn display_format(&self) -> String {
        format!(
            "Running \"{}\" is {}successful",
            self.arguments.join(" "),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            }
        )
    }

    /// Sets the execution result.
    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    /// Retrieves the syntax rules applied to the command.
    fn get_syntax(&self) -> SyntaxForTrait {
        self.syntax.clone()
    }
}
