use cluster::Cluster;
use error::FromElementError;
use register::Register;
use register_properties_group::RegisterPropertiesGroup;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegisterOrCluster {
    Register(Register),
    Cluster(Cluster),
}

impl RegisterOrCluster {
    pub fn from_element(element: &xmltree::Element) -> Result<RegisterOrCluster, FromElementError> {
        match &*element.name {
            "register" => Ok(RegisterOrCluster::Register(try!(Register::from_element(element)))),
            "cluster" => Ok(RegisterOrCluster::Cluster(try!(Cluster::from_element(element)))),
            _ => Err(FromElementError::InvalidFormat),
        }
    }

    pub fn propagate_register_properties(&mut self,
                                         register_properties: &RegisterPropertiesGroup) {
        match *self {
            RegisterOrCluster::Register(ref mut r) => {
                r.register_properties = r.register_properties.merge(register_properties)
            }
            RegisterOrCluster::Cluster(ref mut c) => {
                c.propagate_register_properties(register_properties)
            }
        }
    }
}
