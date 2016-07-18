use device::{Device, PeripheralsMap};
use codegen::error::CodegenError;
use std::io::Write;
use peripheral::Peripheral;
use register::Register;
use register_or_cluster::RegisterOrCluster;
use std::io;


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
        _ => Err(CodegenError::UnsupportedFeature)
    }
}

#[derive(Clone, Debug)]
pub struct CodeGenerator<W: Write> {
    indentation_level: u32,
    out: W,
}

impl<W: Write> CodeGenerator<W> {
    pub fn new(out: W) -> CodeGenerator<W> {
        CodeGenerator {
            indentation_level: 0,
            out: out,
        }
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

    pub fn generate_device(&mut self, d: &Device) -> Result<(), CodegenError> {
        write_line!(self, "#[allow(non_snake_case)]");
        write_line!(self, "#[allow(dead_code)]");
        write_line!(self, "#[allow(non_camel_case_types)]");
        write_line!(self, "mod {} {{", d.name);
        self.indent();
        let peripherals_map = d.peripherals_map();
        for p in &d.peripherals {
            try!(self.generate_peripheral(p, &peripherals_map));
        }
        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_peripheral(&mut self,
                               p: &Peripheral,
                               peripherals_map: &PeripheralsMap)
                               -> Result<(), CodegenError> {
        write_line!(self, "pub mod  {} {{", p.name);
        self.indent();
        write_line!(self, "use core;");
        try!(self.generate_peripheral_registers(p, p, peripherals_map));
        self.deindent();
        write_line!(self, "}}");
        Ok(())
    }

    pub fn generate_peripheral_registers(&mut self,
                                         main_peripheral: &Peripheral,
                                         derived_from: &Peripheral,
                                         peripherals_map: &PeripheralsMap)
                                         -> Result<(), CodegenError> {
        if let Some(ref registers) = derived_from.registers {
            for r in registers {
                if let RegisterOrCluster::Register(ref r) = *r {
                    try!(self.generate_register(r, main_peripheral));
                } else {
                    return Err(CodegenError::UnsupportedFeature);
                }
            }
        }

        if let Some(ref derived_from_name) = derived_from.derived_from {
            try!(self.generate_peripheral_registers(main_peripheral,
                                                    peripherals_map[&**derived_from_name],
                                                    peripherals_map));
        }

        Ok(())
    }

    pub fn generate_register(&mut self, r: &Register, p: &Peripheral) -> Result<(), CodegenError> {
        let address = p.base_address.0 + r.address_offset.0;
        let size = r.register_properties.size.map(|s| s.0).unwrap_or(32);
        let ty = try!(size_to_rust_type(size));
        if r.is_read() {
            write_line!(self, "pub unsafe fn {}() -> {} {{", r.name, ty);
            write_line!(self, "    let ptr = 0x{:x} as *const {};", address, ty);
            write_line!(self, "    core::ptr::read_volatile(ptr)");
            write_line!(self, "}}");
        }

        if r.is_write() {
            write_line!(self, "pub unsafe fn set_{}(value: {}) {{", r.name, ty);
            write_line!(self, "    let ptr = 0x{:x} as *mut {};", address, ty);
            write_line!(self, "    core::ptr::write_volatile(ptr, value)");
            write_line!(self, "}}");
        }

        let ptr_constness = if r.is_write() {
            "mut"
        } else {
            "const"
        };

        write_line!(self, "pub fn {}_ptr() -> *{} {} {{", r.name, ptr_constness, ty);
        write_line!(self, "    0x{:x} as *{} {}", address, ptr_constness, ty);
        write_line!(self, "}}");

        Ok(())
    }
}
