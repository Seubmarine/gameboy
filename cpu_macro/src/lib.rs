extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro]
pub fn uppercase(input : TokenStream) -> TokenStream {
    let original = input.to_string();
    let mut modified = String::new();
    modified.push('"');
    modified.push_str(&original.to_uppercase());
    modified.push('"');
    let token_modified : TokenStream = TokenStream::from_str(&modified).unwrap();
    token_modified
}
