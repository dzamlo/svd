use error::FromElementError;
use std::str::FromStr;

str_enum!{Protection,
    "s" => Secure,
    "n" => NonSecure,
    "p" => Privileged
}
