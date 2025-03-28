use crate::core::save_test_result::CsvTestResult;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum RukiniaResultType {
    TestFail,
    TestSuccess,
}

#[derive(Clone)]
pub struct RukiniaResultEntry {
    pub label: String,
    pub result_type: RukiniaResultType,
}

impl RukiniaResultEntry {
    pub fn new(result_type: RukiniaResultType, label: String) -> Self {
        RukiniaResultEntry { label, result_type }
    }

    pub fn display_result(&self) {
        const GREEN: &str = "\x1b[32m";
        const RED: &str = "\x1b[31m";
        const BOLD: &str = "\x1b[1m";
        const RESET: &str = "\x1b[0m";
        match self.result_type {
            RukiniaResultType::TestSuccess => {
                println!("[{}{}PASS{}] : {}", GREEN, BOLD, RESET, self.label);
            }
            RukiniaResultType::TestFail => {
                println!("[{}{}FAIL{}] : {}", RED, BOLD, RESET, self.label);
            }
        }
    }

    pub async fn write_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = match self.result_type {
            RukiniaResultType::TestSuccess => (&self.label, "SUCCESS"),
            RukiniaResultType::TestFail => (&self.label, "FAIL"),
        };

        let csv_result = CsvTestResult { label, result };
        csv_result
            .append_csv_result(csv_path)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub async fn write_text(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = match self.result_type {
            RukiniaResultType::TestSuccess => (&self.label, "SUCCESS"),
            RukiniaResultType::TestFail => (&self.label, "FAIL"),
        };

        let test_result = format!("{}, {}\n", label, result);

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        let mut file = io::BufWriter::new(file);
        file.write_all(test_result.as_bytes())?;

        Ok(())
    }

    pub async fn write_junit(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = match self.result_type {
            RukiniaResultType::TestSuccess => (&self.label, "SUCCESS"),
            RukiniaResultType::TestFail => (&self.label, "FAIL"),
        };

        let test_result = format!("<testcase name=\"{}\" result=\"{}\" />\n", label, result);

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        let mut file = io::BufWriter::new(file);
        file.write_all(test_result.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct RukiniaError {
    pub label: String,
}

impl fmt::Display for RukiniaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl std::error::Error for RukiniaError {
    fn description(&self) -> &str {
        &self.label
    }
}

impl RukiniaError {
    fn format_message(
        test_command: String,
        input_system_error: String,
        input_system_error_message: String,
    ) -> String {
        format!(
            "Command used : {} | Detailed error : {} | Error message : {}",
            test_command,
            input_system_error,
            input_system_error_message
        )
    }

    pub fn new(
        test_command: String,
        input_system_error: String,
        input_system_error_message: String,
    ) -> Self {
        RukiniaError {
            label: RukiniaError::format_message(
                test_command,
                input_system_error,
                input_system_error_message,
            ),
        }
    }

    pub fn display_result(&self) {
        const RED: &str = "\x1b[31m";
        const BOLD: &str = "\x1b[1m";
        const RESET: &str = "\x1b[0m";
        println!("[{}{}ERROR{}] : {}", RED, BOLD, RESET, self.label);
    }

    pub async fn write_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = (&self.label, "SYSTEM ERROR");
        let csv_result = CsvTestResult { label, result };
        csv_result
            .append_csv_result(csv_path)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub async fn write_text(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = (&self.label, "SYSTEM ERROR");

        let test_result = format!("{}, {}\n", label, result);

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        let mut file = io::BufWriter::new(file);
        file.write_all(test_result.as_bytes())?;

        Ok(())
    }

    pub async fn write_junit(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let (label, result) = (&self.label, "SYSTEM ERROR");

        let test_result = format!("<testcase name=\"{}\" result=\"{}\" />\n", label, result);

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        let mut file = io::BufWriter::new(file);
        file.write_all(test_result.as_bytes())?;

        Ok(())
    }
}
