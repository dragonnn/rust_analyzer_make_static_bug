#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use static_cell::make_static;

fn main() {
    println!("Hello, world!");

    let buff: &mut Vec<u8> = make_static!(Vec::new());
}
