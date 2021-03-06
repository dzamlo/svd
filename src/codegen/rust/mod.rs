use codegen::error::CodegenError;
use device::Device;
use field::{Field, FieldsGroup};
use is_similar::IsSimilarOptions;
use peripheral::{Peripheral, PeripheralsGroup};
use register::Register;
use register_or_cluster::RegisterOrCluster;
use std::fmt::Display;
use std::io;
use std::io::Write;


macro_rules! write_line {
    ( $gen:expr, $($arg:tt)* ) => {{
          try!(($gen).write_indentation());
          try!(writeln!($gen.out, $($arg)*));
    }};
}

fn size_to_rust_type(size: u64) -> Result<&'static str, CodegenError> {
    match size {
        8 => Ok("u8"),
        16 => Ok("u16"),
        32 => Ok("u32"),
        64 => Ok("u64"),
        _ => Err(CodegenError::UnsupportedFeature),
    }
}

#[derive(Clone, Debug)]
pub struct CodeGenerator<W: Write> {
    indentation_level: u32,
    out: W,
    with_field: bool,
    group_fields: bool,
    bool_field: bool,
    group_peripherals: bool,
    with_doc: bool,
}

impl<W: Write> CodeGenerator<W> {
    pub fn new(out: W) -> CodeGenerator<W> {
        CodeGenerator {
            indentation_level: 0,
            out: out,
            with_field: true,
            group_fields: true,
            bool_field: true,
            group_peripherals: true,
            with_doc: true,
        }
    }

    /// If true, generates structs to access the fields of the registers.
    pub fn with_field(mut self, with_field: bool) -> CodeGenerator<W> {
        self.with_field = with_field;
        self
    }

    /// If true, some fields are grouped and can be accessed with an index.
    pub fn group_fields(mut self, group_fields: bool) -> CodeGenerator<W> {
        self.group_fields = group_fields;
        self
    }

    /// If true, use `bool` for single bit fields.
    pub fn bool_field(mut self, bool_field: bool) -> CodeGenerator<W> {
        self.bool_field = bool_field;
        self
    }

    /// If true, some perpherals are grouped together and share a common `struct`
    pub fn group_peripherals(mut self, group_peripherals: bool) -> CodeGenerator<W> {
        self.group_peripherals = group_peripherals;
        self
    }

    /// If true, add doc comment with content from the description fields
    pub fn with_doc(mut self, with_doc: bool) -> CodeGenerator<W> {
        self.with_doc = with_doc;
        self
    }

    fn write_indentation(&mut self) -> Result<(), io::Error> {
        for _ in 0..self.indentation_level {
            try!(write!(self.out, "    "));
        }
        Ok(())
    }

    fn indent(&mut self) {
        self.indentation_level += 1;
    }

    fn deindent(&mut self) {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
    }

    pub fn generate_doc<D: Display>(&mut self, doc: &Option<D>) -> Result<(), CodegenError> {
        if self.with_doc {
            if let Some(ref doc) = *doc {
                write_line!(self, "#[doc = \"{}\"]", doc);
            }
        }
        Ok(())
    }

    pub fn generate_device(&mut self, d: &Device) -> Result<(), CodegenError> {
        write_line!(self, "#[allow(non_snake_case)]");
        write_line!(self, "#[allow(dead_code)]");
        write_line!(self, "#[allow(non_camel_case_types)]");
        try!(self.generate_doc(&Some(&d.description)));
        write_line!(self, "pub mod {} {{", d.name);
        self.indent();

        if self.group_peripherals {
            let mut options = IsSimilarOptions::new();
            options.set_ignore_fields(!self.with_field);
            let (groups, individuals) = PeripheralsGroup::from_peripherals(&d.peripherals,
                                                                           &options);

            for group in &groups {
                try!(self.generate_peripherals_group(group));
            }

            for p in &individuals {
                try!(self.generate_peripheral(p));
            }

        } else {
            for p in &d.peripherals {
                try!(self.generate_peripheral(p));
            }
        }

        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_peripherals_group(&mut self,
                                      pg: &PeripheralsGroup)
                                      -> Result<(), CodegenError> {
        try!(self.generate_doc(&pg.description()));
        write_line!(self, "pub mod {} {{", pg.module_name());
        self.indent();
        write_line!(self, "use core;");
        write_line!(self, "#[derive(Copy, Clone, PartialEq, Eq)]");
        write_line!(self,
                    "pub struct {} {{ pub base_address: usize }}",
                    pg.struct_name());
        for peripheral in pg.peripherals() {
            write_line!(self,
                        "pub const {0}: {1} = {1} {{base_address: {2}}};",
                        peripheral.name,
                        pg.struct_name(),
                        peripheral.base_address.0);
        }

        for peripheral in pg.peripherals() {
            if let Some(ref registers) = peripheral.registers {
                for r in registers {
                    if let RegisterOrCluster::Register(ref r) = *r {
                        try!(self.generate_register_for_group(r, pg));
                    } else {
                        return Err(CodegenError::UnsupportedFeature);
                    }
                }
                break;
            }
        }
        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_peripheral(&mut self, p: &Peripheral) -> Result<(), CodegenError> {
        try!(self.generate_doc(&p.description));
        write_line!(self, "pub mod {} {{", p.name);
        self.indent();
        write_line!(self, "use core;");
        try!(self.generate_peripheral_registers(p));
        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_peripheral_registers(&mut self,
                                         peripheral: &Peripheral)
                                         -> Result<(), CodegenError> {
        if let Some(ref registers) = peripheral.registers {
            for r in registers {
                if let RegisterOrCluster::Register(ref r) = *r {
                    try!(self.generate_register(r, peripheral));
                } else {
                    return Err(CodegenError::UnsupportedFeature);
                }
            }
        }

        Ok(())
    }

    pub fn generate_register(&mut self, r: &Register, p: &Peripheral) -> Result<(), CodegenError> {
        let address = p.base_address.0 + r.address_offset.0;
        let ty = try!(self.generate_fields(r));

        if r.is_read() {
            try!(self.generate_doc(&r.description));
            write_line!(self, "pub unsafe fn read_{}() -> {} {{", r.name, ty);
            write_line!(self, "    let ptr = 0x{:x} as *const {};", address, ty);
            write_line!(self, "    core::ptr::read_volatile(ptr)");
            write_line!(self, "}}");
        }

        if r.is_write() {
            try!(self.generate_doc(&r.description));
            write_line!(self,
                        "pub unsafe fn write_{}<T: Into<{}>>(value: T) {{",
                        r.name,
                        ty);
            write_line!(self, "    let ptr = 0x{:x} as *mut {};", address, ty);
            write_line!(self, "    core::ptr::write_volatile(ptr, value.into())");
            write_line!(self, "}}");
        }

        let ptr_constness = if r.is_write() { "mut" } else { "const" };

        try!(self.generate_doc(&r.description));
        write_line!(self,
                    "pub fn {}_ptr() -> *{} {} {{",
                    r.name,
                    ptr_constness,
                    ty);
        write_line!(self, "    0x{:x} as *{} {}", address, ptr_constness, ty);
        write_line!(self, "}}");

        Ok(())
    }

    pub fn generate_register_for_group(&mut self,
                                       r: &Register,
                                       pg: &PeripheralsGroup)
                                       -> Result<(), CodegenError> {
        let ty = try!(self.generate_fields(r));
        write_line!(self, "impl {} {{", pg.struct_name());
        self.indent();
        if r.is_read() {
            try!(self.generate_doc(&r.description));
            write_line!(self, "pub unsafe fn read_{}(&self) -> {} {{", r.name, ty);
            write_line!(self,
                        "    let ptr = (self.base_address + {}) as * const {};",
                        r.address_offset.0,
                        ty);
            write_line!(self, "    core::ptr::read_volatile(ptr)");
            write_line!(self, "}}");
        }

        if r.is_write() {
            try!(self.generate_doc(&r.description));
            write_line!(self,
                        "pub unsafe fn write_{}<T: Into<{}>>(&self, value: T) {{",
                        r.name,
                        ty);
            write_line!(self,
                        "    let ptr = (self.base_address + {}) as * mut {};",
                        r.address_offset.0,
                        ty);
            write_line!(self, "    core::ptr::write_volatile(ptr, value.into())");
            write_line!(self, "}}");
        }

        let ptr_constness = if r.is_write() { "mut" } else { "const" };
        try!(self.generate_doc(&r.description));
        write_line!(self,
                    "pub fn {}_ptr(&self) -> *{} {} {{",
                    r.name,
                    ptr_constness,
                    ty);
        write_line!(self,
                    "    (self.base_address + 0x{:x}) as *{} {}",
                    r.address_offset.0,
                    ptr_constness,
                    ty);
        write_line!(self, "}}");
        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_fields<'a>(&mut self, r: &'a Register) -> Result<&'a str, CodegenError> {
        let mut ty = try!(size_to_rust_type(r.size()));
        let has_field = match r.fields {
            Some(ref fields) => {
                // If there is only one field and this field use all the bits of the register, do
                // not create a structure for it.
                !fields.is_empty() &&
                (fields.len() > 1 || fields[0].bit_range.width() as u64 != r.size())
            }
            None => false,
        };
        let with_field = self.with_field && has_field;
        if with_field {
            try!(self.generate_doc(&r.description));
            write_line!(self, "pub struct {}(pub {});", r.name, ty);
            write_line!(self, "impl From<{}> for {} {{", ty, r.name);
            write_line!(self, "    fn from(value: {}) -> {} {{", ty, r.name);
            write_line!(self, "        {}(value)", r.name);
            write_line!(self, "    }}");
            write_line!(self, "}}");
            write_line!(self, "impl {} {{", r.name);
            self.indent();
            let fields = r.fields.as_ref().unwrap();
            if self.group_fields {
                let (groups, individuals) = FieldsGroup::from_fields(fields);
                for group in &groups {
                    try!(self.generate_fields_group(group, ty));
                }
                for field in &individuals {
                    try!(self.generate_field(field, ty));
                }
            } else {
                for field in fields {
                    try!(self.generate_field(field, ty));
                }
            }
            self.deindent();
            write_line!(self, "}}");

            ty = &*r.name;
        }

        Ok(ty)
    }

    pub fn generate_bits_get(&mut self, lsb: &str, field_width: u32) -> Result<(), CodegenError> {
        if self.bool_field && field_width == 1 {
            write_line!(self, "(self.0 & (1 << {})) != 0", lsb);
        } else {
            let mask = (1u64 << field_width) - 1;
            write_line!(self, "let mask = {} << {};", mask, lsb);
            write_line!(self, "(self.0 & mask) >> {}", lsb);
        }
        Ok(())
    }

    pub fn generate_bits_set(&mut self, lsb: &str, field_width: u32) -> Result<(), CodegenError> {
        if self.bool_field && field_width == 1 {
            write_line!(self, "if value {{");
            write_line!(self, "    self.0 |= 1 << {};", lsb);
            write_line!(self, "}} else {{");
            write_line!(self, "    self.0 &= !(1 << {});", lsb);
            write_line!(self, "}}");
        } else {
            let mask = (1u64 << field_width) - 1;
            write_line!(self, "let mask = {} << {};", mask, lsb);
            write_line!(self,
                        "self.0 = (self.0 & !mask) | ((value << {}) & mask)",
                        lsb);
        }
        Ok(())
    }

    pub fn generate_field(&mut self, f: &Field, ty: &str) -> Result<(), CodegenError> {
        let msb = f.bit_range.msb;
        let lsb = f.bit_range.lsb;
        let field_width = msb - lsb + 1;
        let ty = if self.bool_field && field_width == 1 {
            "bool"
        } else {
            ty
        };

        if f.is_read() {
            try!(self.generate_doc(&f.description));
            write_line!(self, "pub fn {}(&self) -> {} {{", f.name, ty);
            self.indent();
            write_line!(self, "let lsb = {};", lsb);
            try!(self.generate_bits_get("lsb", field_width));
            self.deindent();
            write_line!(self, "}}");
        }

        if f.is_write() {
            try!(self.generate_doc(&f.description));
            write_line!(self, "pub fn set_{}(&mut self, value: {}) {{", f.name, ty);
            self.indent();
            write_line!(self, "let lsb = {};", lsb);
            try!(self.generate_bits_set("lsb", field_width));
            self.deindent();
            write_line!(self, "}}");
        }
        Ok(())
    }

    pub fn generate_fields_group(&mut self, g: &FieldsGroup, ty: &str) -> Result<(), CodegenError> {
        let ty = if self.bool_field && g.width() == 1 {
            "bool"
        } else {
            ty
        };

        if g.is_read() {
            try!(self.generate_doc(&g.description()));
            write_line!(self,
                        "pub fn {}(&self, index: usize) -> {} {{",
                        g.prefix(),
                        ty);
            self.indent();
            write_line!(self, "assert!(index < {});", g.count());
            write_line!(self,
                        "let lsb = {} + index * {};",
                        g.lsb(),
                        g.lsb_increment());
            try!(self.generate_bits_get("lsb", g.width()));
            self.deindent();
            write_line!(self, "}}");
        }

        if g.is_write() {
            try!(self.generate_doc(&g.description()));
            write_line!(self,
                        "pub fn set_{}(&mut self, index: usize, value: {}) {{",
                        g.prefix(),
                        ty);
            self.indent();
            write_line!(self, "assert!(index < {});", g.count());
            write_line!(self,
                        "let lsb = {} + index * {};",
                        g.lsb(),
                        g.lsb_increment());
            try!(self.generate_bits_set("lsb", g.width()));
            self.deindent();
            write_line!(self, "}}");
        }
        Ok(())
    }
}
