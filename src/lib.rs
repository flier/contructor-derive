//! Registers a function to be called before/after main (if an executable)
//! or when loaded/unloaded (if a dynamic library).
//!
//! **Notes**
//!
//! Use this library is unsafe unless you want to interop directly with a FFI library.
//!
//! Please consider to use the `lazy-static` crate instead of it.
//!
//! Usage
//! =====
//!
//! Add the following dependency to your Cargo manifest...
//!
//! ```toml
//! [dependencies]
//! contructor_derive = "0.1"
//! ```
//!
//! Example
//! =======
//!
//! ```
//! #[macro_use]
//! extern crate contructor_derive;
//!
//! pub static mut RAN: bool = false;
//!
//! #[constructor]
//! extern "C" fn set_ran() {
//!     unsafe { RAN = true }
//! }
//!
//! #[destructor]
//! extern "C" fn reset_ran() {
//!     unsafe { RAN = false }
//! }
//!
//! fn main() {
//!     assert!(unsafe { RAN });
//! }
//! ```

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{Expr, Item, ItemFn, Lit};

/// Registers a function to be called before main (if an executable) or when loaded (if a dynamic library).
#[proc_macro_attribute]
pub fn constructor(args: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).unwrap();

    if let Item::Fn(ref func) = item {
        let priority = parse_priority(args);

        gen_ctor(func, priority).into()
    } else {
        panic!("constructor!{} is only defined for function!");
    }
}

/// Registers a function to be called after main (if an executable) or when unloaded (if a dynamic library).
#[proc_macro_attribute]
pub fn destructor(args: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).unwrap();

    if let Item::Fn(ref func) = item {
        let priority = parse_priority(args);

        gen_dtor(func, priority).into()
    } else {
        panic!("destructor!{} is only defined for function!");
    }
}

fn parse_priority(args: TokenStream) -> Option<u64> {
    if !args.is_empty() {
        let expr: Expr = syn::parse(args).unwrap();

        if let Expr::Lit(lit) = expr {
            if let Lit::Int(n) = lit.lit {
                return Some(n.value());
            }
        }
    }

    None
}

fn gen_ctor(func: &ItemFn, _priority: Option<u64>) -> TokenStream2 {
    let mod_name = Ident::new(&format!("{}_ctor", func.ident), Span::call_site());
    let func_name = &func.ident;

    let ctor = if cfg!(target_os = "linux") {
        quote! {
            #[link_section = ".ctors"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else if cfg!(target_os = "macos") {
        quote! {
            #[link_section = "__DATA,__mod_init_func"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else if cfg!(target_os = "windows") {
        quote! {
            #[link_section = ".CRT$XCU"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else {
        unimplemented!()
    };

    quote!{
        #func

        #[doc(hidden)]
        pub mod #mod_name {
            #ctor
        }
    }
}

fn gen_dtor(func: &ItemFn, _priority: Option<u64>) -> TokenStream2 {
    let mod_name = Ident::new(&format!("{}_dtor", func.ident), Span::call_site());
    let func_name = &func.ident;
    let ctor = if cfg!(target_os = "linux") {
        quote! {
            #[link_section = ".dtors"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else if cfg!(target_os = "macos") {
        quote! {
            #[link_section = "__DATA,__mod_term_func"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else if cfg!(target_os = "windows") {
        quote! {
            #[link_section = ".CRT$XPU"]
            #[no_mangle]
            pub static #func_name: extern fn() = super::#func_name;
        }
    } else {
        unimplemented!()
    };

    quote!{
        #func

        #[doc(hidden)]
        pub mod #mod_name {
            #ctor
        }
    }
}
