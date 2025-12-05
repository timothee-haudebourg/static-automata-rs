use std::hash::Hash;

use iregex::automata::RangeSet;
use proc_macro2::TokenStream;

pub trait Token: iregex::automata::Token + Hash {
	fn rust_type() -> syn::Ident;

	fn from_u8(b: u8) -> Self;

	fn from_char(c: char) -> Option<Self>;

	fn from_u32(v: u32) -> Option<Self>;

	fn singleton(token: Self, case_sensitive: bool) -> RangeSet<Self>;

	fn range(a: Self, b: Self) -> RangeSet<Self>;

	fn rust_pattern(set: &RangeSet<Self>) -> TokenStream;
}

mod byte;
mod char;
