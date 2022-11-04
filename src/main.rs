use std::{collections::BTreeSet, num::NonZeroU32};

use clap::{Parser, ValueEnum};
use pwdgen::{CharSet, Generator, PasswordSpec};
use rand::rngs::OsRng;

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    /// The length of the generated passwords.
    #[arg(short, long, default_value = "40")]
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

    fn to_password_spec(&self) -> PasswordSpec {
        let types_set = BTreeSet::from_iter(self.types.iter().cloned());
        let char_sets = types_set.into_iter().map(|ncs| ncs.to_char_set()).collect();
        PasswordSpec::new(self.length, char_sets)
    }
}

#[derive(ValueEnum, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum NamedCharSet {
    Uppercase,
    Lowercase,
    Digits,
    Symbols,
}

impl NamedCharSet {
    fn to_char_set(&self) -> CharSet {
        let predicate = match self {
            NamedCharSet::Uppercase => char::is_ascii_uppercase,
            NamedCharSet::Lowercase => char::is_ascii_lowercase,
            NamedCharSet::Digits => char::is_ascii_digit,
            NamedCharSet::Symbols => char::is_ascii_punctuation,
        };
        ('\x00'..='\x7f').filter(predicate).collect()
    }
}

fn main() {
    let args = Args::parse();
    let spec = args.to_password_spec();

    if args.verbose {
        eprintln!("Length:     {}", args.length);
        eprintln!("Count:      {}", args.count);
        eprintln!("Entropy:    {:.2}", spec.entropy());
        eprintln!(
            "Characters: ({}) {}",
            spec.char_set_union().len(),
            spec.char_set_union().iter().collect::<String>()
        );
        eprintln!();
    }

    let gen = Generator::new(OsRng, spec);
    gen.take(args.count.get() as usize)
        .for_each(|pwd| println!("{}", pwd));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_char_sets_are_disjoint() {
        let uppercase = NamedCharSet::Uppercase.to_char_set();
        let lowercase = NamedCharSet::Lowercase.to_char_set();
        let digits = NamedCharSet::Digits.to_char_set();
        let symbols = NamedCharSet::Symbols.to_char_set();

        assert!(uppercase.is_disjoint(&lowercase));
        assert!(uppercase.is_disjoint(&digits));
        assert!(uppercase.is_disjoint(&symbols));

        assert!(lowercase.is_disjoint(&digits));
        assert!(lowercase.is_disjoint(&symbols));

        assert!(digits.is_disjoint(&symbols));
    }
}
