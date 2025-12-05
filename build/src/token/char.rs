use iregex::automata::RangeSet;
use proc_macro2::Span;
use quote::quote;

use super::Token;

impl Token for char {
	fn rust_type() -> syn::Ident {
		syn::Ident::new("char", Span::call_site())
	}

	fn from_u8(b: u8) -> Self {
		b as Self
	}

	fn from_char(c: char) -> Option<Self> {
		Some(c)
	}

	fn from_u32(v: u32) -> Option<Self> {
		char::from_u32(v)
	}

	fn singleton(token: Self, case_sensitive: bool) -> RangeSet<Self> {
		let mut set = RangeSet::new();

		if case_sensitive {
			set.insert(token)
		} else {
			for c in token.to_uppercase() {
				set.insert(c)
			}
			for c in token.to_lowercase() {
				set.insert(c)
			}
		}

		set
	}

	fn range(a: Self, b: Self) -> RangeSet<Self> {
		let mut set = RangeSet::new();
		set.insert(a..=b);
		set
	}

	fn rust_pattern(set: &RangeSet<Self>) -> proc_macro2::TokenStream {
		let ranges = set.iter().map(|range| match range.len() {
			0 => panic!("empty range"),
			1 => {
				let a = range.first().unwrap();
				quote! {
					#a
				}
			}
			_ => {
				let a = range.first().unwrap();
				let b = range.last().unwrap();
				quote! {
					#a..=#b
				}
			}
		});

		quote! { #(#ranges)|* }
	}
}
