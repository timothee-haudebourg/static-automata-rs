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

// use iregex::automata::{AnyRange, DFA};
// use quote::quote;

// use crate::charset::CharSet;

// use super::{Token, TokenRange, TokenSet};

// /// Unicode character token.
// impl Token for char {
// 	type Range = AnyRange<char>;

// 	type Set = CharSet;

// 	// type Map<V> = RangeMap<char, V>;

// 	const UNICODE: bool = true;

// 	fn from_u8(b: u8) -> Self {
// 		b as char
// 	}

// 	fn from_char(c: char) -> Option<Self> {
// 		Some(c)
// 	}

// 	fn from_u32(v: u32) -> Option<Self> {
// 		char::from_u32(v)
// 	}

// 	// fn fmt_token(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 	// 	Sanitized(*self).fmt(f)
// 	// }

// 	fn is_ascii(automaton: &DFA<u32, Self::Set>) -> bool {
// 		for transitions in automaton.transitions().values() {
// 			if transitions.keys().any(|set| !set.is_ascii()) {
// 				return false;
// 			}
// 		}

// 		true
// 	}

// 	fn rust_type() -> proc_macro2::TokenStream {
// 		quote!(char)
// 	}

// 	fn rust_string_type() -> proc_macro2::TokenStream {
// 		quote!(str)
// 	}

// 	fn rust_owned_string_type() -> proc_macro2::TokenStream {
// 		quote!(String)
// 	}

// 	fn rust_iterator_method() -> proc_macro2::TokenStream {
// 		quote!(chars())
// 	}

// 	fn rust_as_inner_method() -> proc_macro2::TokenStream {
// 		quote!(as_str)
// 	}

// 	fn rust_into_inner_method() -> proc_macro2::TokenStream {
// 		quote!(into_string)
// 	}

// 	fn rust_inner_as_bytes_method() -> Option<proc_macro2::TokenStream> {
// 		Some(quote!(as_bytes))
// 	}

// 	fn rust_inner_into_bytes_method() -> Option<proc_macro2::TokenStream> {
// 		Some(quote!(into_bytes))
// 	}

// 	fn rust_empty_string() -> proc_macro2::TokenStream {
// 		quote! {
// 			""
// 		}
// 	}
// }

// impl TokenRange<char> for AnyRange<char> {
// 	fn new(a: char, b: char) -> Self {
// 		(a..=b).into()
// 	}

// 	// fn peek(&self) -> Option<char> {
// 	// 	self.first()
// 	// }
// }

// impl TokenSet<char> for CharSet {
// 	fn singleton(token: char, case_sensitive: bool) -> Self {
// 		CharSet::from_char(token, case_sensitive)
// 	}

// 	// fn is_empty(&self) -> bool {
// 	// 	self.is_empty()
// 	// }

// 	// fn len(&self) -> usize {
// 	// 	self.len() as usize
// 	// }

// 	// fn peek(&self) -> Option<char> {
// 	// 	self.first()
// 	// }

// 	// fn intersects_range(&self, range: <char as Token>::Range) -> bool {
// 	// 	self.intersects(range)
// 	// }

// 	// fn merge_with(&mut self, other: Self) {
// 	// 	self.extend(other.ranges())
// 	// }

// 	fn rust_set(&self) -> proc_macro2::TokenStream {
// 		let ranges = self.ranges().map(|range| match range.len() {
// 			0 => panic!("empty range"),
// 			1 => {
// 				let a = range.first().unwrap();
// 				quote! {
// 					#a
// 				}
// 			}
// 			_ => {
// 				let a = range.first().unwrap();
// 				let b = range.last().unwrap();
// 				quote! {
// 					#a..=#b
// 				}
// 			}
// 		});

// 		quote! { #(#ranges)|* }
// 	}
// }

// // impl MergeRef for CharSet {
// // 	fn merge_with_ref(&mut self, other: &Self) {
// // 		self.extend(other.ranges())
// // 	}
// // }

// // impl automaton::DeterminizeLabel for CharSet {
// // 	type Range = AnyRange<char>;

// // 	type Ranges<'a> = charset::Ranges<'a>;

// // 	type RangeMap<V: Clone + PartialEq> = RangeMap<char, V>;

// // 	fn ranges(&self) -> Self::Ranges<'_> {
// // 		CharSet::ranges(self)
// // 	}

// // 	fn insert_range(&mut self, range: <char as Token>::Range) {
// // 		self.insert(range)
// // 	}
// // }

// // impl<V: Clone + PartialEq> automaton::RangeMap<AnyRange<char>, V> for RangeMap<char, V> {
// // 	fn update(&mut self, key: AnyRange<char>, f: impl Fn(Option<&V>) -> Option<V>) {
// // 		RangeMap::update(self, key, f)
// // 	}
// // }
