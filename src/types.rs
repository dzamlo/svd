use error::FromElementError;
use std::str::FromStr;

pub type IdentifierType = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaledNonNegativeInteger(pub u64);

impl FromStr for ScaledNonNegativeInteger {
    type Err = FromElementError;

    fn from_str(s: &str) -> Result<ScaledNonNegativeInteger, FromElementError> {
        let s = if s.starts_with('+') { &s[1..] } else { s };

        let parsed = if s.starts_with('#') {
            let s = &s[1..];
            u64::from_str_radix(s, 2)
        } else if s.starts_with("0x") || s.starts_with("0X") {
            let s = &s[2..];
            u64::from_str_radix(s, 16)
        } else {
            u64::from_str_radix(s, 10)
        };

        match parsed {
            Ok(v) => Ok(ScaledNonNegativeInteger(v)),
            Err(e) => Err(e.into()),
        }

    }
}

#[cfg(test)]
mod tests {
    use error::FromElementError;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn decimal() {
        assert_eq!(Ok(ScaledNonNegativeInteger(10)),
                   ScaledNonNegativeInteger::from_str("10"));
    }

    #[test]
    fn hex() {
        assert_eq!(Ok(ScaledNonNegativeInteger(31)),
                   ScaledNonNegativeInteger::from_str("0x1F"));
        assert_eq!(Ok(ScaledNonNegativeInteger(31)),
                   ScaledNonNegativeInteger::from_str("0X1f"));
    }

    #[test]
    fn binary() {
        assert_eq!(Ok(ScaledNonNegativeInteger(2)),
                   ScaledNonNegativeInteger::from_str("#10"));
    }

    #[test]
    fn invalid() {
        assert_eq!(Err(FromElementError::InvalidFormat),
                   ScaledNonNegativeInteger::from_str(""));
        assert_eq!(Err(FromElementError::InvalidFormat),
                   ScaledNonNegativeInteger::from_str("a"));
        assert_eq!(Err(FromElementError::InvalidFormat),
                   ScaledNonNegativeInteger::from_str("0xg"));
        assert_eq!(Err(FromElementError::InvalidFormat),
                   ScaledNonNegativeInteger::from_str("#2"));
    }
}
