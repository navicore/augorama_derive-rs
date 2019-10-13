extern crate env_logger;
extern crate log;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        trait HelloMac {
            fn hiya();
        }
        impl HelloMac for #name {
            fn hiya() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMac)]
pub fn derive_hellomac(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
