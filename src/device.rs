use access::Access;
use cpu::Cpu;
use peripheral::Peripheral;
use protection::Protection;
use types::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    vendor: Option<String>,
    vendor_id: Option<IdentifierType>,
    name: IdentifierType,
    series: Option<String>,
    version: String,
    description: String,
    license_text: Option<String>,
    cpu: Option<Cpu>,
    header_system_filename: Option<IdentifierType>,
    header_definition_prefix: Option<IdentifierType>,
    address_unit_bits: ScaledNonNegativeInteger,
    width: ScaledNonNegativeInteger,
    size: Option<ScaledNonNegativeInteger>,
    access: Option<Access>,
    protection: Option<Protection>,
    reset_value: Option<ScaledNonNegativeInteger>,
    reset_mask: Option<ScaledNonNegativeInteger>,
    peripherals: Vec<Peripheral>,
}
