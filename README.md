# in_definite
Rust port of [npm indefinite](https://www.npmjs.com/package/indefinite) for deciding when to use A or An.

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

- Is always case sensitive. So, an word like THIS will be considered an acronymn.
