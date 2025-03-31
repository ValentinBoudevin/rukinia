use std::fs;

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
pub struct RukiniaKernelMod {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaKernelMod {
    fn get_rukinia_command() -> &'static str {
        "rukinia_kmod"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_kernel_mod: RukiniaKernelMod = RukiniaKernelMod {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let path = "/proc/modules".to_string();
        let module_name = match rukinia_kernel_mod.arguments.first() {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaKernelMod::get_rukinia_command(),
                        rukinia_kernel_mod.arguments.join(" ")
                    ),
                    "Missing module name argument".to_string(),
                    "No module name provided".to_string(),
                ));
            }
        };

        match fs::read_to_string(&path) {
            Ok(content) => {
                let result = content
                    .lines()
                    .any(|line| line.split_whitespace().next() == Some(module_name));
                if result {
                    rukinia_kernel_mod.result.result_type = RukiniaResultType::TestSuccess;
                }
            }
            Err(e) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaKernelMod::get_rukinia_command(),
                        rukinia_kernel_mod.arguments.join(" ")
                    ),
                    e.to_string(),
                    "Failed to read /proc/modules".to_string(),
                ));
            }
        }

        rukinia_kernel_mod.apply_syntax();
        Ok(rukinia_kernel_mod)
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "Checking kernel module {} {}loaded",
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
