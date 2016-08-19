use cluster::Cluster;
use error::FromElementError;
use is_similar::{IsSimilar, IsSimilarOptions};
use register::Register;
use register_properties_group::RegisterPropertiesGroup;
use types::*;
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

    pub fn derived_from(&self) -> &Option<IdentifierType> {
        match *self {
            RegisterOrCluster::Register(ref r) => &r.derived_from,
            RegisterOrCluster::Cluster(ref c) => &c.derived_from,
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            RegisterOrCluster::Register(ref r) => &*r.name,
            RegisterOrCluster::Cluster(ref c) => &*c.name,
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

    pub fn merge_derived_from(&mut self, derived_from: &RegisterOrCluster) {
        match (self, derived_from) {
            (&mut RegisterOrCluster::Register(ref mut r1),
             &RegisterOrCluster::Register(ref r2)) => r1.merge_derived_from(r2),
            (&mut RegisterOrCluster::Cluster(ref mut c1), &RegisterOrCluster::Cluster(ref c2)) => {
                c1.merge_derived_from(c2)
            }
            _ => (),
        }
    }

    pub fn propagate_derived_from(&mut self) {
        match *self {
            RegisterOrCluster::Register(ref mut r) => {
                r.propagate_derived_from();
            }
            RegisterOrCluster::Cluster(ref mut c) => {
                c.propagate_derived_from();
            }
        }
    }
}

impl<'a, 'b> IsSimilar<&'a RegisterOrCluster> for &'b RegisterOrCluster {
    fn is_similar(self, other: &RegisterOrCluster, options: &IsSimilarOptions) -> bool {
        match (self, other) {
            (&RegisterOrCluster::Register(ref r1), &RegisterOrCluster::Register(ref r2)) => {
                r1.is_similar(r2, options)
            }
            (&RegisterOrCluster::Cluster(ref c1), &RegisterOrCluster::Cluster(ref c2)) => {
                c1.is_similar(c2, options)
            }
            _ => false,
        }
    }
}
