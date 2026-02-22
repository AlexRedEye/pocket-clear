# pocket-clear

Cross-platform terminal clear helper.

## Usage

```rust
use pocket_clear::clear;

fn main() -> std::io::Result<()> {
    clear()?;
    Ok(())
}
