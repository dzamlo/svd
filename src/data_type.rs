use error::FromElementError;
use std::str::FromStr;

str_enum!{DataType,
    "uint8_t" => UInt8,
    "uint16_t" => UInt16,
    "uint32_t" => UInt32,
    "uint64_t" => UInt64,
    "int8_t" => Int8,
    "int16_t" => Int16,
    "int32_t" => Int32,
    "int64_t" => Int64,
    "uint8_t *" => UInt8Ptr,
    "uint16_t *" => UInt16Ptr,
    "uint32_t *" => UInt32Ptr,
    "uint64_t *" => UInt64Ptr,
    "int8_t *" => Int8Ptr,
    "int16_t *" => Int16Ptr,
    "int32_t *" => Int32Ptr,
    "int64_t *" => Int64Ptr,
}
