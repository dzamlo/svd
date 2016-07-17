extern crate xmltree;

#[macro_use]
mod str_enum;
mod utils;

pub mod access;
pub mod address_block;
pub mod bit_range;
pub mod cluster;
pub mod cpu;
pub mod data_type;
pub mod device;
pub mod dim_element_group;
pub mod enumerated_values;
pub mod error;
pub mod field;
pub mod interrupt;
pub mod modified_write_values;
pub mod peripheral;
pub mod protection;
pub mod read_action;
pub mod register;
pub mod register_or_cluster;
pub mod register_properties_group;
pub mod types;

pub mod codegen;
