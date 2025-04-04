use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcessAsync;

pub struct RukiniaHttpReq {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcessAsync for RukiniaHttpReq {
    fn get_rukinia_command() -> &'static str {
        "rukinia_http_request"
    }

    async fn async_new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut runikia_http_req = RukiniaHttpReq {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let url = match runikia_http_req.arguments.first() {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaHttpReq::get_rukinia_command(),
                        runikia_http_req.arguments.join(" ")
                    ),
                    "Missing url name argument".to_string(),
                    "No url name provided".to_string(),
                ));
            }
        };

        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    runikia_http_req.result.result_type = RukiniaResultType::TestSuccess;
                }
            }
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaHttpReq::get_rukinia_command(),
                        runikia_http_req.arguments.join(" ")
                    ),
                    "HTTP REQWEST failed".to_string(),
                    err.to_string(),
                ));
            }
        }
        runikia_http_req.apply_syntax();
        Ok(runikia_http_req)
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "Checking http url {} does {}return 200",
            self.arguments.first().unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            }
        )
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        self.syntax.clone()
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }
}
