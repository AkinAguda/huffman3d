mod ds;
mod functions;
mod utils;
use ds::Node;
use functions::{build_huffman, decode, encode, get_map, get_unique_chars, sort_nodes};
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
extern crate serde_json;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(n: &str) {
    alert(&format!("Hello, {} welcome to huffman3d!", n));
}

#[wasm_bindgen]
pub fn gen_map_json(v: &str) -> String {
    let mut stru = get_unique_chars(&v);
    let sorted = sort_nodes(&mut stru);
    let huffman = build_huffman(sorted).0;
    println!("{:?}", huffman);
    let de_ref = *huffman;
    serde_json::to_string(&de_ref).unwrap()
    // let mut map: HashMap<char, String> = HashMap::new();
    // get_map(&huffman, &mut map, "".to_string());
    // map
}
#[wasm_bindgen]
pub fn get_huffman(v: &str) -> String {
    let mut stru = get_unique_chars(&v);
    let sorted = sort_nodes(&mut stru);
    let huffman = build_huffman(sorted).0;
    let mut map: HashMap<char, String> = HashMap::new();
    get_map(&huffman, &mut map, "".to_string());
    println!("{:?}", map);
    let encoded = encode(&v, &map);
    // encoded
    serde_json::to_string(&map).unwrap()
}
