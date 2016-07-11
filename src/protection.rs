#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protection {
    Secure,
    NonSecure,
    Privileged,
}
