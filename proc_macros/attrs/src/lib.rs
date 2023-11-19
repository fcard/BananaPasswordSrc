extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn web(_: TokenStream, item: TokenStream) -> TokenStream {
  let mut t: TokenStream =
    "#[cfg(target_family = \"wasm\")]".parse().unwrap();
  t.extend(item);
  t
}

#[proc_macro_attribute]
pub fn cli(_: TokenStream, item: TokenStream) -> TokenStream {
  let mut t: TokenStream =
    "#[cfg(not(target_family = \"wasm\"))]".parse().unwrap();
  t.extend(item);
  t
}

