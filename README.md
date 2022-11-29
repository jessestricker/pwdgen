# pwdgen

A simple password generator for the command line.

## Features

- The password is generated using a cryptographically secure pseudo-random
  number generator (CSPRNG).
- Unlike popular password generator websites, this app runs on your machine and
  will never send anything over the network.
- A small code base and only a few dependencies on widely-used crates allow for
  simple auditing.
- Secure default settings, yielding passwords with an entropy of at least 256
  bits

### Missing Features

This application is supposed to
[_do one thing and do it well_](https://en.wikipedia.org/wiki/Unix_philosophy),
so it is intentionally kept small. \
Nevertheless, [some features](https://github.com/jessestricker/pwdgen/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement)
are currently missing.

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

$ pwdgen -l 16 -c 8 -t uppercase -t lowercase -t digits
UOdzHtZGQvVV6dcp
Zyk4VmEkNzDFlvfY
WLc6o8sNfobmdOBX
D7ZXQideI9ByOA3H
tpo7MDaD2TBHu0aZ
8b2Nf3W85oAup02x
m4xotyq20rdohuhm
mbNR1bHjAVjItqqQ
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
