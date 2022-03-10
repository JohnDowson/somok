# Somok - postfix Result/Option wrapping

## Usage:
Add following to your cargo toml:
```toml
somok = "1.0"
```
Then use postfix wrapping as follows:
```rust
use somok::{ Somok, Either };

fn foo() -> Result<Option<Either<String, Vec<u8>>>> {
    String::from("Foobar").left().some().okay()
}
```