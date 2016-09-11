use errors::*;
use std::str::FromStr;

pub type IdentifierType = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaledNonNegativeInteger(pub u64);

impl FromStr for ScaledNonNegativeInteger {
    type Err = Error;

    fn from_str(s: &str) -> Result<ScaledNonNegativeInteger> {
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
    use std::str::FromStr;
    use super::*;

    #[test]
    fn decimal() {
        assert_eq!(Some(ScaledNonNegativeInteger(10)),
                   ScaledNonNegativeInteger::from_str("10").ok());
    }

    #[test]
    fn hex() {
        assert_eq!(Some(ScaledNonNegativeInteger(31)),
                   ScaledNonNegativeInteger::from_str("0x1F").ok());
        assert_eq!(Some(ScaledNonNegativeInteger(31)),
                   ScaledNonNegativeInteger::from_str("0X1f").ok());
    }

    #[test]
    fn binary() {
        assert_eq!(Some(ScaledNonNegativeInteger(2)),
                   ScaledNonNegativeInteger::from_str("#10").ok());
    }

    #[test]
    fn invalid() {
        assert!(ScaledNonNegativeInteger::from_str("").is_err());
        assert!(ScaledNonNegativeInteger::from_str("a").is_err());
        assert!(ScaledNonNegativeInteger::from_str("0xg").is_err());
        assert!(ScaledNonNegativeInteger::from_str("#2").is_err());
    }
}
