use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
use nix::unistd::getgrouplist;
use nix::unistd::Group;
use nix::unistd::User;
use std::collections::HashSet;
use std::ffi::CString;
pub struct RukiniaUserMemberOf {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
}

impl RukiniaProcess for RukiniaUserMemberOf {
    fn get_rukinia_command() -> &'static str {
        "rukinia_user_memberof"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_user_member_of = RukiniaUserMemberOf {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
        };

        let username = match rukinia_user_member_of.arguments.first() {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUserMemberOf::get_rukinia_command(),
                        rukinia_user_member_of.arguments.join(" ")
                    ),
                    "Missing username argument".to_string(),
                    "No username provided".to_string(),
                ));
            }
        };

        let mut user_groups = HashSet::new();

        let c_username = match CString::new(username) {
            Ok(cstr) => cstr,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUserMemberOf::get_rukinia_command(),
                        rukinia_user_member_of.arguments.join(" ")
                    ),
                    "Failed to convert username to CString".to_string(),
                    err.to_string(),
                ));
            }
        };

        let primary_gid = match User::from_name(username) {
            Ok(Some(user)) => user.gid,
            Ok(None) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUserMemberOf::get_rukinia_command(),
                        rukinia_user_member_of.arguments.join(" ")
                    ),
                    "User not found".to_string(),
                    format!("Failed to give user {}", username),
                ));
            }
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUserMemberOf::get_rukinia_command(),
                        rukinia_user_member_of.arguments.join(" ")
                    ),
                    "Failed to retrieve user information".to_string(),
                    err.to_string(),
                ));
            }
        };

        match getgrouplist(&c_username, primary_gid) {
            Ok(groups) => {
                for gid in groups {
                    user_groups.insert(gid);
                }
            }
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaUserMemberOf::get_rukinia_command(),
                        rukinia_user_member_of.arguments.join(" ")
                    ),
                    format!("Failed to retrieve group list for user {}", username),
                    err.to_string(),
                ));
            }
        }

        rukinia_user_member_of.result.result_type = RukiniaResultType::TestSuccess;
        for group_name in rukinia_user_member_of.arguments.iter() {
            if let Ok(Some(group)) = Group::from_name(group_name) {
                if user_groups.contains(&group.gid) {
                    continue;
                }
            }
            rukinia_user_member_of.result.result_type = RukiniaResultType::TestFail;
            break;
        }
        rukinia_user_member_of.apply_syntax();
        Ok(rukinia_user_member_of)
    }

    fn get_result(&self) -> RukiniaResultEntry {
        self.result.clone()
    }

    fn display_format(&self) -> String {
        format!(
            "Checking user {} is {}member of {}",
            self.arguments.first().unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            },
            self.arguments
                .iter()
                .skip(1)
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(" ")
        )
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        self.syntax.clone()
    }
}
