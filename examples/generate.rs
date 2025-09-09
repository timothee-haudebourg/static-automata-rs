use static_automata::{grammar, Validate};

#[grammar(file = "iri.abnf", export("IRI"))]
mod automata {}

#[derive(Validate)]
#[automaton(automata::Iri)]
struct Foo;

fn main() {
	Foo::validate_str("foo");
}
