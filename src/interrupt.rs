use errors::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: i64,
}

impl Interrupt {
    pub fn from_element(element: &xmltree::Element) -> Result<Interrupt> {
        let name = get_mandatory_child_text!(element, "interrupt", "name");
        let description = get_child_text(element, "description");
        let value = get_mandatory_child_text!(element, "interrupt", "value");

        let value = try!(value.parse());
        Ok(Interrupt {
            name: name,
            description: description,
            value: value,
        })

    }
}
