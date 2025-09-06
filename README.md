# in_definite
Rust port of [indefinite](https://www.npmjs.com/package/indefinite) on npm, for deciding which indefinite article to use ('a' or 'an').

Get the indefinite article ('a' or 'an') to match the given word. For example: an umbrella, a user.

[![Rust](https://img.shields.io/badge/rust-1.37.0%2B-blue.svg?maxAge=3600)](https://github.com/mrseanryan/in_definite)
[![](https://img.shields.io/crates/v/in_definite.svg)](https://crates.io/crates/in_definite)
[![](https://docs.rs/in_definite/badge.svg)](https://docs.rs/in_definite)


## Examples

```rust
use in_definite;

let result = in_definite::get_a_or_an("alien");
assert_eq!("an", result);
```

```rust
// Irregular word
let result = in_definite::get_a_or_an("unicorn");
assert_eq!("a", result);
```

```rust
// Title Case
let result = in_definite::get_a_or_an("Ugly");
assert_eq!("An", result);
```

```rust
let result = in_definite::is_an("alien");
assert_eq!(in_definite::Is::An, result);
```

```rust
let result = in_definite::is_an("unicorn");
assert_eq!(in_definite::Is::A, result);
```

note: detecting plurals is not supported.

## Deviations from the original `indefinite`

Upper/lower/mixed case handling: 
- a word like THIS will be always considered to be an acronym.
- normally the result is lower case ('a' or 'an')
- title case is handled as: 'Ugly' -> 'An'
- mixed case is handled as: 'uGly' -> 'an'

Adverbs:
- Handles adverbs with '-ly' in a *generic* manner. Example: "a ubiquitously"

A big thanks to the original authors of [indefinite](https://www.npmjs.com/package/indefinite)!

## Usage

- in_definite is a rust library, but this git repo alo includes a basic command line tool.

See [in_definite_cmd_published](./tests_e2e/in_definite_cmd_published/) for the command line tool which consumes the library.

```
./go-published.sh umbrella
```

Output

```
= in_definite =
===============
an umbrella
```

## Published @ crates.io

https://crates.io/crates/in_definite

## licence = MIT

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
