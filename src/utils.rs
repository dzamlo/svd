use xmltree;

pub struct IsSimilarOptions {
    ignore_fields: bool,
}

impl IsSimilarOptions {
    pub fn new() -> IsSimilarOptions {
        IsSimilarOptions { ignore_fields: false }
    }

    pub fn ignore_fields(&self) -> bool {
        self.ignore_fields
    }

    pub fn set_ignore_fields(&mut self, value: bool) {
        self.ignore_fields = value;
    }
}

/// This trait is for implementing a relaxed equality test. For example, it ignores descriptions
/// and reset values.
pub trait IsSimilar<T> {
    fn is_similar(self, other: T, options: &IsSimilarOptions) -> bool;
}

impl<T1: IntoIterator, T2: IntoIterator> IsSimilar<T2> for T1
    where T1::Item: IsSimilar<T2::Item>
{
    fn is_similar(self, other: T2, options: &IsSimilarOptions) -> bool {
        // This is the same as `self.into_iter().zip(other).all(|(a, b)| a.is_similar(b))` but it
        // also check that the two iterator have the same length
        let mut iter1 = self.into_iter();
        let mut iter2 = other.into_iter();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(a), Some(b)) => {
                    if !a.is_similar(b, options) {
                        return false;
                    }
                }
                (None, None) => return true,
                (Some(_), None) | (None, Some(_)) => return false,
            }
        }
    }
}


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
