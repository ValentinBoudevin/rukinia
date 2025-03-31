use std::fs;

use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
pub struct RukiniaNetworkIsUp {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaNetworkIsUp {
    fn get_rukinia_command() -> &'static str {
        "rukinia_netif_is_up"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_network_is_up = RukiniaNetworkIsUp {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let pid = match rukinia_network_is_up.arguments.first() {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaNetworkIsUp::get_rukinia_command(),
                        rukinia_network_is_up.arguments.join(" ")
                    ),
                    "Missing pid argument".to_string(),
                    "No pid provided".to_string(),
                ));
            }
        };

        let path = format!("/sys/class/net/{}/operstate", pid);

        match fs::read_to_string(&path) {
            Ok(content) => {
                if content.trim() == "up" {
                    rukinia_network_is_up.result.result_type = RukiniaResultType::TestSuccess;
                }
            }
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaNetworkIsUp::get_rukinia_command(),
                        rukinia_network_is_up.arguments.join(" ")
                    ),
                    "Failed to read network interface operstate".to_string(),
                    err.to_string(),
                ));
            }
        }

        rukinia_network_is_up.apply_syntax();
        Ok(rukinia_network_is_up)
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "Checking if interface {} is {}up",
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
