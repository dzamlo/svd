use error::FromElementError;
use std::str::FromStr;

str_enum!{Access,
    "read-only" => ReadOnly,
    "write-only" => WriteOnly,
    "read-write" => ReadWrite,
    "writeOnce" => WriteOnce,
    "read-writeOnce" => ReadWriteOnce
}
