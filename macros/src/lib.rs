use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::ToTokens;

mod validate;

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

#[proc_macro_attribute]
#[proc_macro_error]
pub fn grammar(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut item = syn::parse_macro_input!(input as syn::ItemMod);
	item.content = None;
	item.into_token_stream().into()
}
