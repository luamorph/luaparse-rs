# luaparse-rs

[![docs.rs](https://docs.rs/luaparse-rs/badge.svg)](https://docs.rs/luaparse-rs)
[![crates.io](https://img.shields.io/crates/v/luaparse-rs.svg)](https://crates.io/crates/luaparse-rs)

A multi-version Lua parser written in Rust with support for **Lua 5.1, 5.2, 5.3, 5.4, and Luau** via compile-time version selection.

## Features

- Zero-copy parsing with full AST output
- Compile-time version selection; no runtime overhead
- `no_std` compatible (with `alloc`)

## Installation

```toml
[dependencies]
luaparse-rs = { version = "0.1", features = ["luau"] }
```

Available features: `luau` (default), `lua51`, `lua52`, `lua53`, `lua54`

## Usage

```rust
use luaparse_rs::{Parser, Luau};

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
use luaparse_rs::{Parser, Lua54};

let parser = Parser::<Lua54>::new("local x <const> = 5").unwrap();
let ast = parser.parse().unwrap();
```

## AST Traversal

Walk the syntax tree with the visitor traits, or use the quick closures:

```rust
use luaparse_rs::{Parser, Luau};

let ast = Parser::<Luau>::new("local x = 1").unwrap().parse().unwrap();

ast.for_each_identifier(|ident| {
    println!("{}", ident.name);
});
```

For full control, implement [`Visitor`](https://docs.rs/luaparse-rs/latest/luaparse_rs/ast/visitor/trait.Visitor.html) or [`VisitorMut`](https://docs.rs/luaparse-rs/latest/luaparse_rs/ast/visitor/trait.VisitorMut.html). See the [visitor module docs](https://docs.rs/luaparse-rs/latest/luaparse_rs/ast/visitor/index.html) for examples.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
