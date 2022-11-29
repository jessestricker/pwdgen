# pwdgen

A simple password generator for the command line.

## Features

- The password is generated using a cryptographically secure pseudo-random
  number generator (CSPRNG).
- Unlike popular password generator websites, this app runs on your machine and
  will never send anything over the network.
- A small code base and only a few dependencies on widely-used crates allow for
  simple auditing.

## Usage

```console
$ pwdgen --help
A simple password generator for the command line.

Usage: pwdgen.exe [OPTIONS]

Options:
  -l, --length <LENGTH>  The length of the generated passwords [default: 40]
  -c, --count <COUNT>    The number of passwords to generate [default: 1]
  -t, --types <TYPES>    The sets of character to sample from [default: uppercase lowercase digits symbols] [possible values: uppercase, lowercase, digits, symbols]
  -v, --verbose          Print extra information about the generated passwords to standard error
  -h, --help             Print help information
  -V, --version          Print version information

$ pwdgen --verbose
Length:     40
Count:      1
Entropy:    262.18 bits
Characters: (94) !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~

w5!:W|zuJO~HT/B0$v4}"mrO3$gMI![Kbn)tV0#f
```

## Disclaimer

I personally use this app as my password generator, as I don't trust
closed-source websites on generating passwords.

My main motivation with this repository is to have a best-practices template for
Rust projects available as a reference for new projects. I will also use this
repository as an integration test for better CI/CD on GitHub.

If you want to use this app for yourself, please check the source code in
advance. I welcome any issue reports and pull requests on this GitHub
repository.
