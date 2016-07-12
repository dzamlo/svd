use std::num::ParseIntError;
use std::string;
use xmltree;

#[derive(Debug, PartialEq, Eq)]
pub enum FromElementError {
    /// A mandatory field is missing
    MissingField,
    /// A value as an invalid format
    InvalidFormat,
}

impl From<ParseIntError> for FromElementError {
    fn from(_e: ParseIntError) -> FromElementError {
        FromElementError::InvalidFormat
    }
}

impl From<string::ParseError> for FromElementError {
    fn from(_e: string::ParseError) -> FromElementError {
        FromElementError::InvalidFormat
    }
}

#[derive(Debug)]
pub enum ParseError {
    /// Error while parsing the xml
    XmlParse(xmltree::ParseError),
    /// Error while converting an `xmlree::Element` into an svd struct
    FromElement(FromElementError),
}

impl From<xmltree::ParseError> for ParseError {
    fn from(e: xmltree::ParseError) -> ParseError {
        ParseError::XmlParse(e)
    }
}

impl From<FromElementError> for ParseError {
    fn from(e: FromElementError) -> ParseError {
        ParseError::FromElement(e)
    }
}
