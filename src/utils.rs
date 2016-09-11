use xmltree;

pub fn extract_prefix(name: &str) -> (&str, Option<usize>) {
    let prefix_end = name.rfind(|c: char| !c.is_digit(10));
    match prefix_end {
        Some(prefix_end) => (&name[..prefix_end + 1], name[prefix_end + 1..].parse().ok()),
        None => ("", name.parse().ok()),
    }
}

pub fn get_child_text(element: &xmltree::Element, name: &str) -> Option<String> {
    element.get_child(name).map(|child| child.text.clone().unwrap_or_else(String::new))
}

macro_rules! merge_option_field {
    ( $field:expr,  $field_derived_from:expr) => {
        if $field.is_none() {
            $field = $field_derived_from.clone();
        }
    };
}

macro_rules! get_mandatory_child_text {
    ( $xml_element:expr, $element_name:expr, $field_name:expr) => {
        if let Some(value) = get_child_text($xml_element, $field_name) {
            value
        } else {
            return Err(ErrorKind::MissingField($element_name, $field_name).into())
        }
    };
}


#[cfg(test)]
mod tests {
    use super::extract_prefix;

    #[test]
    fn test_extract_prefix() {
        assert_eq!(("", None), extract_prefix(""));
        assert_eq!(("Foo", None), extract_prefix("Foo"));
        assert_eq!(("Foo", Some(123)), extract_prefix("Foo123"));
        assert_eq!(("Foo123Bar", Some(456)), extract_prefix("Foo123Bar456"));
        assert_eq!(("", Some(456)), extract_prefix("456"));
    }
}
