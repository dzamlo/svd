# svd

The goal of this crate is to parse System View Description (SVD) files and generate rust code to access the registers.

SVD files describes mcu and their peripherals.

## Documentation

The code is not really documented at all. But the structures are pretty much the same than in an SVD file. You should reffer to the CMSIS-SVD documentation.

## Usage

You will first need the `.svd` file describing your mcu. You can try the following location to find it:
 * http://www.arm.com/products/processors/cortex-m/cortex-microcontroller-software-interface-standard.php on the CMSIS-SVD tab,
 * the website of the manufacturer of your mcu,
 * the folders of the software provided by the manufacturer of your mcu,
 * https://github.com/posborne/cmsis-svd/tree/master/data

Add the following to you `Cargo.toml` file:
```
[build-dependencies]
svd =  { git = "https://github.com/dzamlo/svd.git" }
```

And then something like the following as your `build.rs`:
```rust
extern crate svd;

use std::env;
use std::fs::File;
use std::path::Path;
use svd::codegen::rust;

const SVD_FILENAME: &'static str = "STM32F7x7.svd";
const OUT_FILENAME: &'static str = "STM32F7x7.rs";

fn main() {
   let f_in = File::open(SVD_FILENAME).unwrap();
   let out_dir = env::var("OUT_DIR").unwrap();
   let dest_path = Path::new(&out_dir).join(OUT_FILENAME);
   let mut f_out = File::create(&dest_path).unwrap();

   let d = svd::device::Device::from_reader(f_in).unwrap();
   let mut code_generator = rust::CodeGenerator::new(&mut f_out);
   code_generator.generate_device(&d).unwrap();
}
```

And finally in your code:
```rust
include!{concat!(env!("OUT_DIR"), "/STM32F7x7.rs")}
```

You can then use the various registers of your device. For exemple to enable the GPIOB on an STM32 device:
```rust
unsafe {
    let mut value = STM32F7x7::RCC::read_AHB1ENR();
    value.set_GPIOBEN(1);
    STM32F7x7::RCC::write_AHB1ENR(value);
}
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
