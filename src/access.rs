use error::FromElementError;
use std::str::FromStr;

str_enum!{Access,
    "read-only" => ReadOnly,
    "write-only" => WriteOnly,
    "read-write" => ReadWrite,
    "writeOnce" => WriteOnce,
    "read-writeOnce" => ReadWriteOnce
}

impl Access {
    pub fn is_read(&self) -> bool {
        match *self {
            Access::ReadOnly | Access::ReadWrite | Access::ReadWriteOnce => true,
            Access::WriteOnly | Access::WriteOnce => false,
        }
    }

    pub fn is_write(&self) -> bool {
        match *self {
            Access::ReadOnly => false,
            Access::WriteOnly | Access::ReadWrite | Access::WriteOnce | Access::ReadWriteOnce => {
                true
            }
        }
    }
}
