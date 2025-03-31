use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;

pub struct RukiniaFalse {
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaFalse {
    fn get_rukinia_command() -> &'static str {
        "false"
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        self.syntax.clone()
    }

    fn new(_arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_false = RukiniaFalse {
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, "false".to_string()),
        };
        rukinia_false.apply_syntax();
        Ok(rukinia_false)
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "{}false",
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            }
        )
    }
}
