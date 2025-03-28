use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;

pub struct RukiniaUser {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaUser {
    fn get_rukinia_command() -> &'static str {
        "rukinia_user"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_user = RukiniaUser {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let username = match rukinia_user.arguments.get(0) {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUser::get_rukinia_command(),
                        rukinia_user.arguments.join(" ")
                    ),
                    "Missing username argument".to_string(),
                    "No username provided".to_string(),
                ));
            }
        };

        let file = match File::open("/etc/passwd") {
            Ok(file) => file,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUser::get_rukinia_command(),
                        rukinia_user.arguments.join(" ")
                    ),
                    "Failed to open /etc/passwd".to_string(),
                    err.to_string(),
                ));
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    if line_content.starts_with(username) {
                        rukinia_user.result.result_type = RukiniaResultType::TestSuccess;
                        break;
                    }
                }
                Err(err) => {
                    return Err(RukiniaError::new(
                        format!(
                            "{} {}",
                            RukiniaUser::get_rukinia_command(),
                            rukinia_user.arguments.join(" ")
                        ),
                        "Failed to read in /etc/passwd".to_string(),
                        err.to_string(),
                    ));
                }
            }
        }

        rukinia_user.apply_syntax();
        return Ok(rukinia_user);
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "Checking user {} {}exists",
            self.arguments.get(0).unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            }
        );
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        return self.syntax.clone();
    }
}
