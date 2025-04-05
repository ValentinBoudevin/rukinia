//! Module containing all Rukinia tasks and their core traits.
//!
//! This module defines:
//! - The enum `RukiniaAllTasks` that represents all available tasks
//! - The traits `RukiniaProcess` and `RukiniaProcessAsync` for synchronous and asynchronous task processing
//! - Implementations for executing tasks and converting from string representations

use std::future::Future;

use crate::core::syntax::SyntaxForTrait;

#[cfg(feature = "kernel")]
use crate::tasks::kernel::{rukinia_kconf::RukiniaKernelConf,rukinia_kmod::RukiniaKernelMod,rukinia_kthread::RukiniaKernelThread};

#[cfg(feature = "network")]
use crate::tasks::network::{rukinia_http_request::RukiniaHttpReq, rukinia_netif_has_ip::RukiniaNetifHasIp, rukinia_network_is_up::RukiniaNetworkIsUp};

#[cfg(feature = "user")]
use crate::tasks::user::{rukinia_user::RukiniaUser, rukinia_user_memberof::RukiniaUserMemberOf, rukinia_group::RukiniaGroup};

#[cfg(feature = "filesystem")]
use crate::tasks::filesystem::{rukinia_cmd::RukiniaCmd,rukinia_symlink::RukiniaSymlink};

use crate::tasks::bool::{rukinia_false::RukiniaFalse,rukinia_true::RukiniaTrue};

use crate::core::rukinia_result::RukiniaError;
use crate::core::rukinia_result::RukiniaResultEntry;
use crate::core::rukinia_result::RukiniaResultType;

use std::str::FromStr;

/// Enum representing all available Rukinia tasks.
///
/// Each variant corresponds to a specific test or check that Rukinia can perform.
#[derive(Clone)]
pub enum RukiniaAllTasks {
    #[cfg(feature = "network")]
    NetworkIsUp,
    #[cfg(feature = "network")]
    NetifHasIp,
    #[cfg(feature = "network")]
    HttpReq,
    #[cfg(feature = "user")]
    User,
    #[cfg(feature = "user")]
    Group,
    #[cfg(feature = "user")]
    UserMemberOf,
    #[cfg(feature = "kernel")]
    KernelMod,
    #[cfg(feature = "kernel")]
    KernelThread,
    #[cfg(feature = "kernel")]
    KernelConf,
    #[cfg(feature = "filesystem")]
    Symlink,
    #[cfg(feature = "filesystem")]
    Cmd,
    True,
    False,
}

impl RukiniaAllTasks {
    /// Execute the task with given arguments and syntax rules.
    ///
    /// # Arguments
    /// * `arguments` - Vector of string arguments for the task
    /// * `syntax` - Syntax rules to apply to the task
    ///
    /// # Returns
    /// `Result<RukiniaResultEntry, RukiniaError>` containing the test result or error
    ///
    /// # Examples
    /// ```rust,ignore
    /// let task = RukiniaAllTasks::NetworkIsUp;
    /// let result = task.execute(vec!["eth0".to_string()], syntax).await;
    /// ```
    pub async fn execute(
        self,
        arguments: Vec<String>,
        syntax: SyntaxForTrait,
    ) -> Result<RukiniaResultEntry, RukiniaError> {
        match self {
            #[cfg(feature = "network")]
            RukiniaAllTasks::NetworkIsUp => match RukiniaNetworkIsUp::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "network")]
            RukiniaAllTasks::NetifHasIp => match RukiniaNetifHasIp::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "network")]
            RukiniaAllTasks::HttpReq => match RukiniaHttpReq::async_new(arguments, syntax).await {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "user")]
            RukiniaAllTasks::User => match RukiniaUser::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "user")]
            RukiniaAllTasks::Group => match RukiniaGroup::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "user")]
            RukiniaAllTasks::UserMemberOf => match RukiniaUserMemberOf::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "kernel")]
            RukiniaAllTasks::KernelConf => match RukiniaKernelConf::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "kernel")]
            RukiniaAllTasks::KernelMod => match RukiniaKernelMod::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "kernel")]
            RukiniaAllTasks::KernelThread => match RukiniaKernelThread::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "filesystem")]
            RukiniaAllTasks::Symlink => match RukiniaSymlink::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            #[cfg(feature = "filesystem")]
            RukiniaAllTasks::Cmd => match RukiniaCmd::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            RukiniaAllTasks::True => match RukiniaTrue::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
            RukiniaAllTasks::False => match RukiniaFalse::new(arguments, syntax) {
                Ok(rukinia_valid) => Ok(rukinia_valid.get_result()),
                Err(rukinia_error) => Err(rukinia_error),
            },
        }
    }
}

impl FromStr for RukiniaAllTasks {
    type Err = ();

    /// Convert from string representation to RukiniaAllTasks variant.
    ///
    /// # Arguments
    /// * `s` - String representing the task name
    ///
    /// # Returns
    /// `Result<Self, ()>` with the matching variant or error if no match found
    ///
    /// # Examples
    /// ```rust,ignore
    /// let task = RukiniaAllTasks::from_str("network_is_up").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s == RukiniaNetworkIsUp::get_rukinia_command() => Ok(RukiniaAllTasks::NetworkIsUp),
            _ if s == RukiniaKernelConf::get_rukinia_command() => Ok(RukiniaAllTasks::KernelConf),
            
            _ if s == RukiniaNetifHasIp::get_rukinia_command() => Ok(RukiniaAllTasks::NetifHasIp),
            
            _ if s == RukiniaHttpReq::get_rukinia_command() => Ok(RukiniaAllTasks::HttpReq),
            #[cfg(feature = "user")]
            _ if s == RukiniaUser::get_rukinia_command() => Ok(RukiniaAllTasks::User),
            #[cfg(feature = "user")]
            _ if s == RukiniaGroup::get_rukinia_command() => Ok(RukiniaAllTasks::Group),
            #[cfg(feature = "user")]
            _ if s == RukiniaUserMemberOf::get_rukinia_command() => Ok(RukiniaAllTasks::UserMemberOf),
            _ if s == RukiniaKernelMod::get_rukinia_command() => Ok(RukiniaAllTasks::KernelMod),
            _ if s == RukiniaKernelThread::get_rukinia_command() => Ok(RukiniaAllTasks::KernelThread),
            #[cfg(feature = "filesystem")]
            _ if s == RukiniaSymlink::get_rukinia_command() => Ok(RukiniaAllTasks::Symlink),
            _ if s == RukiniaTrue::get_rukinia_command() => Ok(RukiniaAllTasks::True),
            _ if s == RukiniaFalse::get_rukinia_command() => Ok(RukiniaAllTasks::False),
            #[cfg(feature = "filesystem")]
            _ if s == RukiniaCmd::get_rukinia_command() => Ok(RukiniaAllTasks::Cmd),
            _ => Err(()),
        }
    }
}

/// Trait defining synchronous Rukinia task processing.
///
/// All synchronous tasks must implement this trait to be compatible with Rukinia's execution engine.
pub trait RukiniaProcess {
    /// Get the command string that identifies this task.
    fn get_rukinia_command() -> &'static str;

    /// Create a new instance of the task.
    ///
    /// # Arguments
    /// * `arguments` - Vector of string arguments
    /// * `syntax` - Syntax rules to apply
    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized;

    /// Get the current result of the task.
    fn get_result(&self) -> RukiniaResultEntry;

    /// Set the result of the task.
    fn set_result(&mut self, result: RukiniaResultEntry);

    /// Get the display format for the task's result.
    fn display_format(&self) -> String;

    /// Get the syntax rules for this task.
    fn get_syntax(&self) -> SyntaxForTrait;

    /// Apply syntax transformations to the result.
    ///
    /// This handles NOT operators and AS label replacements.    
    fn apply_syntax(&mut self) {
        let mut new_rukinia_result_type: RukiniaResultType = self.get_result().result_type;
        if self.get_syntax().contains_not() {
            match self.get_result().result_type {
                RukiniaResultType::TestSuccess => {
                    new_rukinia_result_type = RukiniaResultType::TestFail
                }
                RukiniaResultType::TestFail => {
                    new_rukinia_result_type = RukiniaResultType::TestSuccess
                }
            }
        }
        self.set_result(RukiniaResultEntry {
            result_type: new_rukinia_result_type,
            label: self
                .get_syntax()
                .get_as()
                .unwrap_or_else(|| self.display_format()),
        });
    }
}

/// Trait defining asynchronous Rukinia task processing.
///
/// All asynchronous tasks must implement this trait to be compatible with Rukinia's execution engine.
pub trait RukiniaProcessAsync {
    /// Get the command string that identifies this task.
    fn get_rukinia_command() -> &'static str;

    /// Asynchronously create a new instance of the task.
    ///
    /// # Arguments
    /// * `arguments` - Vector of string arguments
    /// * `syntax` - Syntax rules to apply    
    fn async_new(
        arguments: Vec<String>,
        syntax: SyntaxForTrait,
    ) -> impl Future<Output = Result<Self, RukiniaError>> + Send
    where
        Self: Sized;

    /// Get the current result of the task.    
    fn get_result(&self) -> RukiniaResultEntry;

    /// Get the display format for the task's result.    
    fn display_format(&self) -> String;

    /// Set the result of the task.    
    fn set_result(&mut self, result: RukiniaResultEntry);

    /// Get the syntax rules for this task.    
    fn get_syntax(&self) -> SyntaxForTrait;

    /// Apply syntax transformations to the result.
    ///
    /// This handles NOT operators and AS label replacements.    
    fn apply_syntax(&mut self) {
        let mut new_rukinia_result_type: RukiniaResultType = self.get_result().result_type;
        if self.get_syntax().contains_not() {
            match self.get_result().result_type {
                RukiniaResultType::TestSuccess => {
                    new_rukinia_result_type = RukiniaResultType::TestFail
                }
                RukiniaResultType::TestFail => {
                    new_rukinia_result_type = RukiniaResultType::TestSuccess
                }
            }
        }
        self.set_result(RukiniaResultEntry {
            result_type: new_rukinia_result_type,
            label: self
                .get_syntax()
                .get_as()
                .unwrap_or_else(|| self.display_format()),
        });
    }
}
