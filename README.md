# luaparse

A multi-version Lua parser written in Rust with support for **Lua 5.1, 5.2, 5.3, 5.4, and Luau** via compile-time feature flags.

## Features

- Zero-copy parsing with full AST output
- Compile-time version selection; no runtime overhead
- `no_std` compatible (with `alloc`)

## Installation

```toml
[dependencies]
luaparse = { version = "0.1", features = ["luau"] }
```

Available features: `luau` (default), `lua51`, `lua52`, `lua53`, `lua54`

## Usage

```rust
use luaparse-rs::{Parser, Luau};

let input = r#"
local function greet(name: string): string
    return `hello {name}`
end
"#;

let parser = Parser::<Luau>::new(input).unwrap();
let ast = parser.parse().unwrap();
println!("{:#?}", ast);
```

Switch versions at compile time:

```rust
use luaparse-rs::{Parser, Lua54};

let parser = Parser::<Lua54>::new("local x <const> = 5").unwrap();
let ast = parser.parse().unwrap();
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
