/// Command line interface to build deterministic finite automata module files
/// from ABNF grammars.
///
/// Part of the [`static-automata`](https://crates.io/crates/static-automata)
/// framework.
use build_automata::build_automata_with;
use clap::Parser;

/// Generate Rust automata module files.
#[derive(Parser)]
struct Args {
	/// Do not actually generate the automaton module files.
	#[arg(short, long)]
	dry_run: bool,
}

fn main() {
	let args = Args::parse();

	let options = build_automata::Options {
		dry_run: args.dry_run,
	};

	build_automata_with(&options);
}
