

use wasm_bindgen::prelude::*;
use asuran_chunker::*;
use std::{io::Cursor, convert::TryInto};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//https://github.com/rustwasm/wasm-bindgen/issues/111
#[wasm_bindgen]
pub fn compute_chunks_buzhash(data: Vec<u8>, mask: u32) -> Vec<u32> {
    let cursor = Cursor::new(data);
    let bh = BuzHash::new(0, 4095, mask);
    let chunks = bh.chunk(cursor).map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut offsets: Vec<u32> = Vec::new();
    let mut _current: u32 = 0;
    for v in chunks {
        _current = _current + v.len() as u32;
        offsets.push(_current);
    }
    offsets
}

#[wasm_bindgen]
pub fn compute_chunks_fastcdc(data: Vec<u8>,  min_size: u32, avg_size: u32, max_size: u32) -> Vec<u32> {
    let cursor = Cursor::new(data);
    let bh = FastCDC { min_size: min_size.try_into().unwrap(), avg_size: avg_size.try_into().unwrap(), max_size: max_size.try_into().unwrap()};
    let chunks = bh.chunk(cursor).map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut offsets: Vec<u32> = Vec::new();
    let mut _current: u32 = 0;
    for v in chunks {
        _current = _current + v.len() as u32;
        offsets.push(_current);
    }
    offsets
}