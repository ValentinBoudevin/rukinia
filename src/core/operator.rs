use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogicalOperator {
    And,
    Or,
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            LogicalOperator::And => "and",
            LogicalOperator::Or => "or",
        })
    }
}

impl FromStr for LogicalOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(LogicalOperator::And),
            "or" => Ok(LogicalOperator::Or),
            _ => Err(()),
        }
    }
}
