//! # learning macros
//!
//!
extern crate augorama_derive;
use augorama_derive::HelloMac;

#[derive(HelloMac)]
struct Pancakes;

#[test]
fn derive_trait_says_hiya() {
    Pancakes::hiya();
}
