extern crate svd;

use svd::codegen::rust;

fn main() {
    let input = std::io::stdin();
    let mut output = std::io::stdout();
    let d = svd::device::Device::from_reader(input).unwrap();
    let mut code_generator = rust::CodeGenerator::new(&mut output);
    code_generator.generate_device(&d).unwrap();
}
