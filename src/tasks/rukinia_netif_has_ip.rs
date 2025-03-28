use crate::core::rukinia_result::*;
use crate::core::syntax::SyntaxForTrait;
use crate::tasks::task::RukiniaProcess;
use nix::ifaddrs::getifaddrs;
pub struct RukiniaNetifHasIp {
    pub arguments: Vec<String>,
    pub syntax: SyntaxForTrait,
    result: RukiniaResultEntry,
    ip_version: String,
    extra_flags: String,
}

impl RukiniaProcess for RukiniaNetifHasIp {
    fn get_rukinia_command() -> &'static str {
        "rukinia_netif_has_ip"
    }

    fn new(arguments: Vec<String>, syntax: SyntaxForTrait) -> Result<Self, RukiniaError>
    where
        Self: Sized,
    {
        let mut rukinia_net_if_has_ip = RukiniaNetifHasIp {
            arguments,
            syntax,
            result: RukiniaResultEntry::new(RukiniaResultType::TestFail, String::new()),
            ip_version: String::from("-4"),
            extra_flags: String::new(),
        };

        for arg in rukinia_net_if_has_ip.arguments.iter().skip(1) {
            if arg == "-6" {
                rukinia_net_if_has_ip.ip_version = String::from("-6");
            } else {
                rukinia_net_if_has_ip.extra_flags.push_str(arg);
                rukinia_net_if_has_ip.extra_flags.push(' ');
            }
        }

        let iface_name = match rukinia_net_if_has_ip.arguments.get(0) {
            Some(name) => name.trim_matches('"'),
            None => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaNetifHasIp::get_rukinia_command(),
                        rukinia_net_if_has_ip.arguments.join(" ")
                    ),
                    "Missing interface name argument".to_string(),
                    "No interface name provided".to_string(),
                ));
            }
        };

        let ifaddrs = match getifaddrs() {
            Ok(addrs) => addrs,
            Err(err) => {
                return Err(RukiniaError::new(
                    format!(
                        "{} {}",
                        RukiniaNetifHasIp::get_rukinia_command(),
                        rukinia_net_if_has_ip.arguments.join(" ")
                    ),
                    "Failed to retrieve network interfaces".to_string(),
                    err.to_string(),
                ));
            }
        };

        let result = ifaddrs
            .into_iter()
            .filter_map(|ifa| {
                // Only process interfaces with the correct name
                if ifa.interface_name == iface_name {
                    Some((ifa.flags, ifa.address))
                } else {
                    None
                }
            })
            .any(|(flags, address)| {
                let flags_str = format!("{}", flags.to_string()).to_lowercase();
                let extra_flags_lower = rukinia_net_if_has_ip.extra_flags.trim().to_lowercase();
                let flags_contains_extra = flags_str.contains(&extra_flags_lower);

                if !rukinia_net_if_has_ip.extra_flags.trim().is_empty() && !flags_contains_extra {
                    return false;
                }

                if let Some(addr) = address {
                    if addr.as_sockaddr_in().is_some() && rukinia_net_if_has_ip.ip_version == "-4" {
                        return true;
                    }
                    if addr.as_sockaddr_in6().is_some() && rukinia_net_if_has_ip.ip_version == "-6"
                    {
                        return true;
                    }
                }
                false
            });

        if result {
            rukinia_net_if_has_ip.result.result_type = RukiniaResultType::TestSuccess;
        }
        rukinia_net_if_has_ip.apply_syntax();
        return Ok(rukinia_net_if_has_ip);
    }

    fn get_result(&self) -> RukiniaResultEntry {
        return self.result.clone();
    }

    fn display_format(&self) -> String {
        return format!(
            "Checking interface {} has {}ipv{} assigned {}",
            self.arguments.get(0).unwrap(),
            if self.syntax.contains_not() {
                "not "
            } else {
                ""
            },
            self.ip_version,
            self.extra_flags
        );
    }

    fn set_result(&mut self, result: RukiniaResultEntry) {
        self.result = result;
    }

    fn get_syntax(&self) -> SyntaxForTrait {
        return self.syntax.clone();
    }
}
