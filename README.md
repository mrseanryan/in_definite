# in_definite
Rust port of [indefinite](https://www.npmjs.com/package/indefinite) on npm, for deciding when to use A or An.

Get 'a' or 'an' to match the given word.

## Examples

```rust
use in_definite;

let result = in_definite::get_a_or_an("alien");
assert_eq!("an", result);
```

```rust
let result = in_definite::get_a_or_an("unicorn");
assert_eq!("a", result);
```

```rust
let result = in_definite::is_an("alien");
assert_eq!(true, result);
```

```rust
let result = in_definite::is_an("unicorn");
assert_eq!(false, result);
```

## Devations from the original `indefinite`

- Is always case sensitive. So, a word like THIS will be always considered to be an acronymn.
- Handles adverbs with '-ly' in a *generic* manner. Example: "a ubiquitously"

A big thanks to the original authors of [indefinite](https://www.npmjs.com/package/indefinite)!

## Published @ crates.io

https://crates.io/crates/in_definite

## licence = MIT

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
