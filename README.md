# Types Fuckery

A BrainFuck interpreter fully implemented in the Rust typesystem.

## Usage

See [`main.rs`](src/main.rs)

```rust
use types_fuckery::types_fuckery;

type Output = types_fuckery! {
    program: { /* ... brainfuck program ... */ },
    input:   { /* ... program input */ }
};
```

Output is then a cons list containing the program output in reverse order.

Non trivial program require raising the type system recursion limit.

## `no_std`? more like `no_code`

The library not only is `no_std`, it do not contain any code at all. The only
code is in [`main.rs`](src/main.rs) and [the module connected to
it](src/static_display.rs) to display the cons list.

## License

MIT, but please send me a mail if you find a use for this thing.
