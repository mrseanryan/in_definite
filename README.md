# in_definite
Rust port of [npm indefinite](https://www.npmjs.com/package/indefinite) for deciding when to use A or An.

Get 'a' or 'an' to match the given word.

## Examples

```rust
let result = in_definite::get_a_or_an("alien");
assert_eq!("an", result);
```

```rust
let result = in_definite::get_a_or_an("unicorn");
assert_eq!("a", result);
```
