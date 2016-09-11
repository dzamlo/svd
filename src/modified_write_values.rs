use errors::*;
use std::str::FromStr;

str_enum!{ModifiedWriteValues,
    "oneToClear" => OneToClear,
    "oneToSet" => OneToSet,
    "oneToToggle" => OneToToggle,
    "zeroToClear" => ZeroToClear,
    "zeroToSet" => ZeroToSet,
    "zeroToToggle" => ZeroToToggle,
    "clear" => Clear,
    "set" => Set,
    "modify" => Modify,
}
