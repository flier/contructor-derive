contructor_derive [![Build Status](https://api.travis-ci.org/flier/contructor-derive.svg?branch=master)](https://travis-ci.org/flier/contructor-derive) [![Latest Version](https://img.shields.io/crates/v/contructor_derive.svg)](https://crates.io/crates/contructor_derive) [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/contructor_derive/)
====

Registers a function to be called before/after main (if an executable)
or when loaded/unloaded (if a dynamic library).

**Notes**

Use this library is unsafe unless you want to interop directly with a FFI library.

Please consider to use the `lazy-static` crate instead of it.

## Usage

Add the following dependency to your Cargo manifest...

```toml
[dependencies]
contructor_derive = "0.1.0"
```

## Example

```rust
#[macro_use]
extern crate contructor_derive;

pub static mut RAN: bool = false;

#[constructor]
extern "C" fn set_ran() {
    unsafe { RAN = true }
}

#[destructor]
extern "C" fn reset_ran() {
    unsafe { RAN = false }
}

fn main() {
    assert!(unsafe { RAN });
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
