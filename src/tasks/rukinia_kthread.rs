use std::fs;

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
pub struct RukiniaKernelThread {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaKernelThread {
    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_kernel_thread = RukiniaKernelThread {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let process_name = match rukinia_kernel_thread.arguments.get(0) {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaKernelThread::get_rukinia_command(),
                        rukinia_kernel_thread.arguments.join(" ")
                    ),
                    "Missing process name argument".to_string(),
                    "No process name provided".to_string(),
                ));
            }
        };

        match fs::read_dir("/proc") {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let pid_dir = path.file_name().and_then(|s| s.to_str());
                    if !path.is_dir()
                        || pid_dir.is_none()
                        || !pid_dir.unwrap().chars().all(char::is_numeric)
                    {
                        continue;
                    }

                    let status_file = path.join("status");

                    if let Ok(status) = fs::read_to_string(&status_file) {
                        for line in status.lines() {
                            if line.starts_with("Name:\t") {
                                if line.split_whitespace().nth(1) == Some(process_name) {
                                    rukinia_kernel_thread.result.result_type =
                                        RukiniaResultType::TestSuccess;
                                }
                            }
                            if line.starts_with("VmSize:") {
                                rukinia_kernel_thread.result.result_type =
                                    RukiniaResultType::TestFail;
                                break;
                            }
                        }
                    }
                    if rukinia_kernel_thread.result.result_type == RukiniaResultType::TestSuccess {
                        break;
                    }
                }
            }
            Err(e) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaKernelThread::get_rukinia_command(),
                        rukinia_kernel_thread.arguments.join(" ")
                    ),
                    e.to_string(),
                    "Failed to read /proc/modules".to_string(),
                ));
            }
        }
        rukinia_kernel_thread.apply_syntax();
        return Ok(rukinia_kernel_thread);
    }

    fn get_rukinia_command() -> &'static str {
        "rukinia_kthread"
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "Checking kernel thread {} is {}running",
            self.arguments.get(0).unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            },
        );
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        return self.syntax.clone();
    }
}
