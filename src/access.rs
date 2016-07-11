#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    WriteOnce,
    ReadWriteOnce,
}
