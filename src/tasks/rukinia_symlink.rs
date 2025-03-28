use std::fs;

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
use std::path::Path;
pub struct RukiniaSymlink {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaSymlink {
    fn get_rukinia_command() -> &'static str {
        "rukinia_symlink"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_symlink = RukiniaSymlink {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail,String::new()),
        };

        let link_path = match rukinia_symlink.arguments.get(0) {
            Some(path) => Path::new(path),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaSymlink::get_rukinia_command(),
                        rukinia_symlink.arguments.join(" ")
                    ),
                    "Missing link path argument".to_string(),
                    "No link path provided".to_string(),
                ));
            }
        };

        let target_path = match rukinia_symlink.arguments.get(1) {
            Some(path) => Path::new(path),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaSymlink::get_rukinia_command(),
                        rukinia_symlink.arguments.join(" ")
                    ),
                    "Missing target path argument".to_string(),
                    "No target path provided".to_string(),
                ));
            }
        };

        if !link_path.is_symlink() {
            rukinia_symlink.apply_syntax();
            return Ok(rukinia_symlink);
        }

        let link_target = match fs::read_link(link_path) {
            Ok(path) => path,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaSymlink::get_rukinia_command(),
                        rukinia_symlink.arguments.join(" ")
                    ),
                    "Failed to read symbolic link".to_string(),
                    err.to_string(),
                ));
            }
        };

        let canonical_link_target = match fs::canonicalize(&link_target) {
            Ok(path) => path,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaSymlink::get_rukinia_command(),
                        rukinia_symlink.arguments.join(" ")
                    ),
                    "Failed to canonicalize link target".to_string(),
                    err.to_string(),
                ));
            }
        };

        let canonical_target = match fs::canonicalize(target_path) {
            Ok(path) => path,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaSymlink::get_rukinia_command(),
                        rukinia_symlink.arguments.join(" ")
                    ),
                    "Failed to canonicalize target target".to_string(),
                    err.to_string(),
                ));
            }
        };

        if canonical_link_target == canonical_target {
            rukinia_symlink.result.result_type = RukiniaResultType::TestSuccess;
        }

        rukinia_symlink.apply_syntax();
        return Ok(rukinia_symlink);
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "Checking link {} does {}point to {}",
            self.arguments.get(0).unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            },
            self.arguments.get(1).unwrap()
        );
    }
    
    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        return self.syntax.clone();
    }
}
