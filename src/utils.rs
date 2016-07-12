use xmltree;

pub fn get_child_text(element: &xmltree::Element, name: &str) -> Option<String> {
    element.get_child(name).map(|child| child.text.clone().unwrap_or_else(String::new))
}
