// use iregex::automata::{AnyRange, DFA};
// use quote::quote;

// use crate::byteset::{self, ByteSet};

// use super::{Token, TokenRange, TokenSet};

// impl Token for u8 {
// 	type Range = AnyRange<u8>;

// 	type Set = ByteSet;

// 	// type Map<V> = RangeMap<u8, V>;

// 	const UNICODE: bool = false;

// 	fn from_u8(b: u8) -> Self {
// 		b
// 	}
// //
// 	fn from_char(c: char) -> Option<Self> {
// 		if c.is_ascii() {
// 			Some(c as u8)
// 		} else {
// 			None
// 		}
// 	}

// 	fn from_u32(v: u32) -> Option<Self> {
// 		if v <= 0xff {
// 			Some(v as u8)
// 		} else {
// 			None
// 		}
// 	}

// 	// fn fmt_token(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 	// 	Sanitized(*self).fmt(f)
// 	// }

// 	fn rust_type() -> proc_macro2::TokenStream {
// 		quote!(u8)
// 	}

// 	fn rust_string_type() -> proc_macro2::TokenStream {
// 		quote!([u8])
// 	}

// 	fn rust_owned_string_type() -> proc_macro2::TokenStream {
// 		quote!(Vec<u8>)
// 	}

// 	fn rust_iterator_method() -> proc_macro2::TokenStream {
// 		quote!(iter().copied())
// 	}

// 	fn rust_as_inner_method() -> proc_macro2::TokenStream {
// 		quote!(as_bytes)
// 	}

// 	fn rust_into_inner_method() -> proc_macro2::TokenStream {
// 		quote!(into_bytes)
// 	}

// 	fn is_ascii(automaton: &DFA<u32, Self::Set>) -> bool {
// 		for transitions in automaton.transitions().values() {
// 			if transitions.keys().any(|set| !set.is_ascii()) {
// 				return false;
// 			}
// 		}

// 		true
// 	}

// 	fn rust_inner_as_ascii_method_body() -> Option<proc_macro2::TokenStream> {
// 		Some(quote!(unsafe { ::core::str::from_utf8_unchecked(&self.0) }))
// 	}

// 	fn rust_empty_string() -> proc_macro2::TokenStream {
// 		quote! {
// 			b""
// 		}
// 	}
// }

// impl TokenRange<u8> for AnyRange<u8> {
// 	fn new(a: u8, b: u8) -> Self {
// 		(a..=b).into()
// 	}
// }

// impl TokenSet<u8> for ByteSet {
// 	fn singleton(token: u8, case_sensitive: bool) -> Self {
// 		ByteSet::from_u8(token, case_sensitive)
// 	}

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

// // impl MergeRef for ByteSet {
// // 	fn merge_with_ref(&mut self, other: &Self) {
// // 		self.extend(other.ranges())
// // 	}
// // }

// // impl automaton::DeterminizeLabel for ByteSet {
// // 	type Range = AnyRange<u8>;

// // 	type Ranges<'a> = byteset::Ranges<'a>;

// // 	type RangeMap<V: Clone + PartialEq> = RangeMap<u8, V>;

// // 	fn ranges(&self) -> Self::Ranges<'_> {
// // 		ByteSet::ranges(self)
// // 	}

// // 	fn insert_range(&mut self, range: <u8 as Token>::Range) {
// // 		self.insert(range)
// // 	}
// // }

// // impl<V: Clone + PartialEq> automaton::RangeMap<AnyRange<u8>, V> for RangeMap<u8, V> {
// // 	fn update(&mut self, key: AnyRange<u8>, f: impl Fn(Option<&V>) -> Option<V>) {
// // 		RangeMap::update(self, key, f)
// // 	}
// // }
