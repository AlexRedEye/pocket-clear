# pocket-clear

Cross-platform terminal clear helper.

To be entirely transparent I used AI to make most of this code since I'm still learning.
I don't know if this is the safest way to do this or if the most effiecient.
As I learn and start to understand concepts I will update this if needed.
I simply wanted an easy way to clear the terminal while I make silly CLI games.

## Installing as a dependancy
```bash
cargo add pocket-clear
```

## Usage

```rust
use pocket_clear::clear;

fn main() -> std::io::Result<()> {
    clear()?;
    Ok(())
}
