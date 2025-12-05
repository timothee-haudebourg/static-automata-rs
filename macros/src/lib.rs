/// Macros for the [`static-automata`](https://crates.io/crates/static-automata)
/// library. This is not supposed to be used directly.
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::ToTokens;

mod validate;

/// Derive grammar validation functions.
///
/// This derive macro generates the two functions `validate_bytes` and
/// `validate_str` using the provided automaton function.
///
/// The two generated functions are `const` compatible.
#[proc_macro_derive(Validate, attributes(automaton))]
#[proc_macro_error]
pub fn derive_validate(input: TokenStream) -> TokenStream {
	let item = syn::parse_macro_input!(input as syn::DeriveInput);
	match validate::derive(item) {
		Ok(tokens) => tokens.into(),
		Err(e) => {
			let span = e.span();
			abort!(span, e)
		}
	}
}

/// Grammar module annotation.
///
/// This attribute is used to annotate a `mod foo {}` module with information
/// about the grammar it represents. The macro itself doesn't generate any code,
/// it will just replace it with an external module declaration `mod foo;`.
/// It is used as a marker for the Command Line Interface to pick up and
/// generate the external module file.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn grammar(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut item = syn::parse_macro_input!(input as syn::ItemMod);
	item.content = None;
	item.into_token_stream().into()
}
