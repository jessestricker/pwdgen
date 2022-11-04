# pwdgen

A simple password generator for the command line.

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
