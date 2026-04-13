use wasm_bindgen::prelude::*;
use alloc::string::String;

#[wasm_bindgen]
pub fn parse_luau(input: &str) -> String {
    parse_with_version::<crate::Luau>(input)
}

#[wasm_bindgen]
pub fn parse_lua51(input: &str) -> String {
    parse_with_version::<crate::Lua51>(input)
}

#[wasm_bindgen]
pub fn parse_lua52(input: &str) -> String {
    parse_with_version::<crate::Lua52>(input)
}

#[wasm_bindgen]
pub fn parse_lua53(input: &str) -> String {
    parse_with_version::<crate::Lua53>(input)
}

#[wasm_bindgen]
pub fn parse_lua54(input: &str) -> String {
    parse_with_version::<crate::Lua54>(input)
}

#[wasm_bindgen]
pub fn parse(input: &str, version: &str) -> String {
    match version {
        "luau" => parse_luau(input),
        "5.1" | "lua51" => parse_lua51(input),
        "5.2" | "lua52" => parse_lua52(input),
        "5.3" | "lua53" => parse_lua53(input),
        "5.4" | "lua54" => parse_lua54(input),
        _ => alloc::format!("Error: unknown version '{}'. Use: luau, 5.1, 5.2, 5.3, 5.4", version),
    }
}

fn parse_with_version<V: crate::marker::LuaVersion>(input: &str) -> String {
    use crate::Parser;
    
    match Parser::<V>::new(input) {
        Ok(parser) => {
            match parser.parse() {
                Ok(ast) => {
                    alloc::format!("{:#?}", ast)
                }
                Err(e) => {
                    alloc::format!("Parse Error: {:#?}", e)
                }
            }
        }
        Err(e) => {
            alloc::format!("Lexer Error: {:#?}", e)
        }
    }
}