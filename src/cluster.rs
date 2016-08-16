use dim_element_group::DimElementGroup;
use error::FromElementError;
use register_or_cluster::RegisterOrCluster;
use register_properties_group::RegisterPropertiesGroup;
use types::*;
use utils::{get_child_text, IsSimilar, IsSimilarOptions};
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cluster {
    pub derived_from: Option<IdentifierType>,
    pub dim_element: DimElementGroup,
    pub name: IdentifierType,
    pub alternate_cluster: Option<IdentifierType>,
    pub header_struct_name: Option<IdentifierType>,
    pub address_offset: ScaledNonNegativeInteger,
    pub register_properties: RegisterPropertiesGroup,
    pub registers: Vec<RegisterOrCluster>,
}

impl Cluster {
    pub fn from_element(element: &xmltree::Element) -> Result<Cluster, FromElementError> {
        let derived_from = element.attributes.get("derivedFrom").cloned();
        let dim_element = try!(DimElementGroup::from_element(element));
        let name = get_child_text(element, "name");
        let alternate_cluster = get_child_text(element, "alternateCluster");
        let header_struct_name = get_child_text(element, "headerStructName");
        let address_offset = get_child_text(element, "addressOffset");
        let register_properties = try!(RegisterPropertiesGroup::from_element(element));
        let registers: Result<Vec<_>, FromElementError> = element.children
            .iter()
            .filter(|e| e.name == "register" || e.name == "cluster")
            .map(RegisterOrCluster::from_element)
            .collect();
        let registers = try!(registers);

        if name.is_none() || address_offset.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let address_offset = try!(address_offset.unwrap().parse());
            Ok(Cluster {
                derived_from: derived_from,
                dim_element: dim_element,
                name: name,
                alternate_cluster: alternate_cluster,
                header_struct_name: header_struct_name,
                address_offset: address_offset,
                register_properties: register_properties,
                registers: registers,
            })

        }
    }

    pub fn propagate_register_properties(&mut self,
                                         register_properties: &RegisterPropertiesGroup) {
        self.register_properties = self.register_properties.merge(register_properties);
        for r_or_c in &mut self.registers {
            r_or_c.propagate_register_properties(&self.register_properties);
        }
    }
}

impl<'a, 'b> IsSimilar<&'a Cluster> for &'b Cluster {
    fn is_similar(self, other: &Cluster, options: &IsSimilarOptions) -> bool {
        self.name == other.name && self.address_offset == other.address_offset &&
        self.register_properties == other.register_properties &&
        self.registers.is_similar(&other.registers, options)
    }
}
