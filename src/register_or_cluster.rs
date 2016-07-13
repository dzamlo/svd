use cluster::Cluster;
use error::FromElementError;
use register::Register;
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
}
