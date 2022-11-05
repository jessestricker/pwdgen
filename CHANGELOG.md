# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2022-11-05

### Documentation

- Add README and LICENSE
- Clarify where verbose output is written
- Add usage information

### Features

- Add CLI
- Add password spec and generator
- Generate passwords with CLI
- Set default length to reach 256 bits entropy
- Add unit to printed entropy

### Miscellaneous Tasks

- Add release script

### Testing

- Assert that named char sets are disjoint
- Assert password spec invariants
- Assert generated password length

### Build

- Add Cargo project
- Use Cargo target auto-discovery
- Reset MSRV
- Depend on `clap`
- Bump MSRV to 1.60
- Depend on `rand`

### Ci

- Add GitHub Dependabot config
- Add GitHub workflow
- Add release workflow
- Fix GitHub expression strings
- Fix artifact upload path
- Get name as lowercase with `awk`
- Fix archive creation for Windows
- Publish GitHub release

<!-- generated by git-cliff -->