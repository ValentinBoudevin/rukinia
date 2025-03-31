use csv::WriterBuilder;
use serde::Serialize;
use std::fmt;
use std::fs::OpenOptions;
use std::path::Path;
#[derive(Serialize)]
pub struct CsvTestResult<'a> {
    #[serde(rename = "TEST MESSAGE")]
    pub label: &'a str,
    #[serde(rename = "RESULT")]
    pub result: &'a str,
}

#[derive(Clone)]
pub struct ResultFormat {
    pub format: FormatOutput,
    pub path: String,
}

#[derive(PartialEq, Clone)]
pub enum FormatOutput {
    Csv,
    TextFile,
    JUnit,
}

impl fmt::Display for FormatOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormatOutput::Csv => write!(f, "Csv"),
            FormatOutput::TextFile => write!(f, "TextFile"),
            FormatOutput::JUnit => write!(f, "JUnit"),
        }
    }
}

impl CsvTestResult<'_> {
    pub async fn append_csv_result(&self, csv_path: &str) -> Result<(), csv::Error> {
        let file_exists = Path::new(csv_path).exists();

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(csv_path)?;

        let mut wtr = WriterBuilder::new()
            .has_headers(!file_exists)
            .from_writer(file);

        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }
}
