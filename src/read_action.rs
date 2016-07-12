use error::FromElementError;
use std::str::FromStr;

str_enum!{ReadAction,
    "clear" => Clear,
    "set" => Set,
    "modify" => Modify,
    "modifyExternal" => ModifyExternal,
}
