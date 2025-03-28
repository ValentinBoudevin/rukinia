use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;

pub struct RukiniaTrue {
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaTrue {
    fn get_rukinia_command() -> &'static str {
        "true"
    }

    fn new(_arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_true = RukiniaTrue {
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestSuccess, "true".to_string()),
        };
        rukinia_true.apply_syntax();
        return Ok(rukinia_true);
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "{}true",
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
