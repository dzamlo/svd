use std;
use xmltree;

error_chain! {
    foreign_links {
        xmltree::ParseError, XmlParseError;
        std::num::ParseIntError, ParseIntError;
    }

    errors {
        MissingField(element_name: &'static str, field_name: &'static str) {
            description("missing field")
            display("missing field '{}' in a '{}'", field_name, element_name)
        }

        UnexpectedValue(expected: &'static str, actual: String) {
            description("unexpected value")
            display("expected: {}, got: {}", expected, actual)
        }
    }
}
