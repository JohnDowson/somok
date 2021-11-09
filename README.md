# Somok - postfix Result/Option wrapping

## Usage:
Add following to your cargo toml:
```toml
somok = "1.0"
```
Then use postfix wrapping as follows:
```rust
use somok::Somok;

fn foo() -> Result<Option<String>> {
    String::from("Foobar").some().okay()
}
```