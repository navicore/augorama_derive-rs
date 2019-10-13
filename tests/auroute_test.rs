//! # learning macros
//!
//!
extern crate augorama_derive;
use augorama_derive::HelloMac;

#[derive(HelloMac)]
struct Pancakes;

// todo: make this test real!
#[test]
fn auroute_macro_creates_routes() {
    Pancakes::hiya();
}
