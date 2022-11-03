use std::num::NonZeroU32;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    /// The length of the generated passwords.
    #[arg(short, long, default_value = "16")]
    length: NonZeroU32,

    /// The number of passwords to generate.
    #[arg(short, long, default_value = "1")]
    count: NonZeroU32,

    /// The sets of character to sample from.
    #[arg(short, long, value_enum, default_values_t = Args::DEFAULT_TYPES)]
    types: Vec<NamedCharSet>,

    /// Print extra information about the generated passwords.
    #[arg(short, long)]
    verbose: bool,
}

impl Args {
    const DEFAULT_TYPES: [NamedCharSet; 4] = [
        NamedCharSet::Uppercase,
        NamedCharSet::Lowercase,
        NamedCharSet::Digits,
        NamedCharSet::Symbols,
    ];
}

#[derive(ValueEnum, Clone, Debug)]
pub enum NamedCharSet {
    Uppercase,
    Lowercase,
    Digits,
    Symbols,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
}
