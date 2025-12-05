use crate::token::Token;
use iregex::automata::{DFA, RangeSet};

pub mod abnf;

#[derive(Debug, thiserror::Error)]
pub enum GrammarError {
	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("invalid ABNF grammar: {0}")]
	Abnf(::abnf::error::ParseError),

	#[error("undefined non-terminal `{0}`")]
	UndefinedNonTerminal(String),

	#[error("redefined non-terminal `{0}`")]
	RedefinedNonTerminal(String),

	#[error("grammar is not regular")]
	NonRegular,

	#[error("no entry point")]
	NoEntryPoint,

	#[error("unsupported unicode character token `{0}`")]
	UnsupportedCharToken(char),

	#[error("unsupported 32-bit interger token `{0:#x}`")]
	UnsupportedU32Token(u32),

	#[error("missing grammar")]
	MissingGrammar,

	#[error("unknown grammar format")]
	UnknownGrammarFormat,

	#[error("invalid documentation attribute")]
	InvalidDocumentation,

	#[error("inconsistent grammar type")]
	InconsistentGrammarType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GrammarType {
	Abnf,
}

pub enum Grammar<T: Token> {
	Abnf(abnf::Grammar<T>),
}

impl<T: Token> Grammar<T> {
	pub fn new(
		ty: GrammarType,
		data: &str,
		entry_point: Option<&str>,
	) -> Result<Self, GrammarError> {
		match ty {
			GrammarType::Abnf => Ok(Self::Abnf(abnf::Grammar::new(data, entry_point)?)),
		}
	}

	pub fn build_automaton(self) -> DFA<u32, RangeSet<T>> {
		match self {
			Self::Abnf(g) => g.build_dfa(),
		}
	}
}
