/// Command line interface to build deterministic finite automata module files
/// from ABNF grammars.
///
/// Part of the [`static-automata`](https://crates.io/crates/static-automata)
/// framework.
use build_automata::build_automata_with;
use clap::Parser;

/// Generate Rust automata module files.
#[derive(Parser)]
#[command(bin_name = "cargo")]
struct Args {
	#[clap(subcommand)]
	command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
	BuildAutomata(BuildAutomata),
}

#[derive(Parser)]
struct BuildAutomata {
	/// Do not actually generate the automaton module files.
	#[arg(short, long)]
	dry_run: bool,
}

fn main() {
	let args = Args::parse();

	match args.command {
		Command::BuildAutomata(args) => {
			let options = build_automata::Options {
				dry_run: args.dry_run,
			};

			build_automata_with(&options);
		}
	}
}
