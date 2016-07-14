use error::FromElementError;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: i64,
}

impl Interrupt {
    pub fn from_element(element: &xmltree::Element) -> Result<Interrupt, FromElementError> {
        let name = get_child_text(element, "name");
        let description = get_child_text(element, "description");
        let value = get_child_text(element, "value");

        if name.is_none() || value.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let value = try!(value.unwrap().parse());
            Ok(Interrupt {
                name: name,
                description: description,
                value: value,
            })
        }
    }
}
