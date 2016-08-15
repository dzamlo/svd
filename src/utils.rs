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
