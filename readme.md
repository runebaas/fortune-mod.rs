# Fortune-mod (in rust)

This is a clone of the original `fortune` command found on many unix systems

It's primarily an educational project in which i aim to completely reimplement the command.

## Progress

- [x] Print a fortune from a fortunes file
- [x] Specify a cutoff for what is considered a short fortune (the `-n` flag)
- [x] Specify whether a short or a long fortune should be printed (the `-l` flag)
- [x] Wait n amount of seconds before the program exits (the `-w` flag)
- [x] Allow to filter with a pattern (the `-m` flag)
- [x] Ignore case for the pattern (the `-i` flag)
- [x] Allow multiple fortune cookie files
- [ ] Implement the `-f` flag

## Installing

Prerequisites:
* rust
* clang
* pkg-config
* openssl

Build:
```sh
cargo install https://github.com/runebaas/fortune-mod.rs.git 
```

## Fortune files

Place your fortune files in one of the following locations:

* *nix
  * /usr/share/fortune
  * /usr/share/games/fortunes
  * ./fortunes
* windows
  * ./fortunes
  * ./cookies
