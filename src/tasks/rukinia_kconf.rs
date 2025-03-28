use std::fs;

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;

pub struct RukiniaKernelConf {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaKernelConf {
    fn get_rukinia_command() -> &'static str {
        "rukinia_kconf"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukiniakernelconfg = RukiniaKernelConf {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let path = format!("/proc/config.gz");

        match fs::read_to_string(&path) {
            Ok(content) => {
                let test_succed = content.trim() == rukiniakernelconfg.arguments.get(0).unwrap();
                if test_succed {
                    rukiniakernelconfg.result.result_type = RukiniaResultType::TestSuccess;
                }
            }
            Err(e) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaKernelConf::get_rukinia_command(),
                        rukiniakernelconfg.arguments.join(" ")
                    ),
                    e.to_string(),
                    "Failed to read /proc/config.gz".to_string(),
                ));
            }
        }
        rukiniakernelconfg.apply_syntax();
        return Ok(rukiniakernelconfg);
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "Checking kernel config {} {}set to {}",
            self.arguments.get(0).unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            },
            self.result.label
        );
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        return self.syntax.clone();
    }
}
