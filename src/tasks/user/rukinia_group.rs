use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;

pub struct RukiniaGroup {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaGroup {
    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_group = RukiniaGroup {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let group_name = match rukinia_group.arguments.first() {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaGroup::get_rukinia_command(),
                        rukinia_group.arguments.join(" ")
                    ),
                    "Missing group name argument".to_string(),
                    "No group name provided".to_string(),
                ));
            }
        };

        let file = match File::open("/etc/group") {
            Ok(file) => file,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaGroup::get_rukinia_command(),
                        rukinia_group.arguments.join(" ")
                    ),
                    "Failed to open /etc/group".to_string(),
                    err.to_string(),
                ));
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    if line_content.starts_with(group_name) {
                        rukinia_group.result.result_type = RukiniaResultType::TestSuccess;
                        break;
                    }
                }
                Err(err) => {
                    return Err(RukiniaError::new(
                        format!(
                            "{} {}",
                            RukiniaGroup::get_rukinia_command(),
                            rukinia_group.arguments.join(" ")
                        ),
                        "Failed to read in /etc/group".to_string(),
                        err.to_string(),
                    ));
                }
            }
        }

        rukinia_group.apply_syntax();
        Ok(rukinia_group)
    }

    fn get_rukinia_command() -> &'static str {
        "rukinia_group"
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "Checking group {} {}exists",
            self.arguments.first().unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            }
        )
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        self.syntax.clone()
    }
}
