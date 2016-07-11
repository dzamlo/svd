use error::FromElementError;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protection {
    Secure,
    NonSecure,
    Privileged,
}

impl FromStr for Protection {
    type Err = FromElementError;

    fn from_str(s: &str) -> Result<Protection, FromElementError> {
        match s {
            "s" => Ok(Protection::Secure),
            "n" => Ok(Protection::NonSecure),
            "p" => Ok(Protection::Privileged),
            _ => Err(FromElementError::InvalidFormat),
        }

    }
}
