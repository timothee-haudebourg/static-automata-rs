use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("invalid `automaton` attribute")]
	InvalidAttribute(Span),

	#[error("missing automaton")]
	MissingAutomaton,

	#[error(transparent)]
	Syntax(#[from] syn::Error),
}

impl Error {
	pub fn span(&self) -> Span {
		match self {
			Self::InvalidAttribute(s) => *s,
			Self::MissingAutomaton => Span::call_site(),
			Self::Syntax(e) => e.span(),
		}
	}
}

pub fn derive(input: DeriveInput) -> Result<TokenStream, Error> {
	let ident = &input.ident;

	let mut automaton: Option<syn::Path> = None;
	for attr in input.attrs {
		if attr.path().is_ident("automaton") {
			match attr.meta {
				syn::Meta::List(meta) => {
					automaton = Some(syn::parse2(meta.tokens)?);
				}
				_ => return Err(Error::InvalidAttribute(attr.span())),
			}
		}
	}

	let automaton = automaton.ok_or(Error::MissingAutomaton)?;

	Ok(quote! {
		impl #ident {
			pub const fn validate_str(input: &str) -> bool {
				#automaton::validate_str(input)
			}

			pub const fn validate_bytes(input: &[u8]) -> bool {
				#automaton::validate_bytes(input)
			}
		}
	})
}
