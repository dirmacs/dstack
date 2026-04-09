# dstack-memory

Pluggable memory layer for AI development stacks.

## Backends

- **FileProvider** — JSON-based local storage at `~/.dstack/memory/`
- **ErukaProvider** — REST API backend via [Eruka](https://eruka.dirmacs.com) context engine

## Usage

```rust
use dstack_memory::{MemoryProvider, FileProvider, Field};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = FileProvider::new("~/.dstack/memory")?;

    // Save a field
    provider.save(Field {
        path: "projects/myapp/learnings/auth-fix".to_string(),
        value: "JWT refresh token rotation prevents session hijacking".to_string(),
        ..Default::default()
    }).await?;

    // Search memory
    let results = provider.search("auth").await?;
    for field in results {
        println!("{}: {}", field.path, field.value);
    }

    Ok(())
}
```

## Features

- `file` — Enable FileProvider (default)
- `eruka` — Enable ErukaProvider (requires running Eruka instance)

```toml
[dependencies]
dstack-memory = { version = "0.3", features = ["file", "eruka"] }
```

## License

MIT
