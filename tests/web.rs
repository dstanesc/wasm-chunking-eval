//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate web_sys;

use wasm_chunking_eval::{compute_chunks_buzhash, compute_chunks_fastcdc};
use wasm_chunking_eval::log;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn test_data() -> Vec<u8> {
    let data: Vec<u8> = br"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut sit amet bibendum lorem. In pharetra quis felis vel placerat. Aenean eget elementum turpis. Phasellus dolor sem, facilisis a suscipit nec, ullamcorper quis lorem. Proin faucibus purus nec diam feugiat, ac pulvinar purus venenatis. Suspendisse ultrices vestibulum tortor laoreet condimentum. Ut id sapien porta, fringilla urna vitae, congue mi. Praesent eleifend tempus justo, eu volutpat mauris vehicula at. Phasellus ornare congue tortor a tristique. Praesent tristique dolor bibendum sem tempus, ac finibus felis egestas. Nulla nec massa porta, pulvinar urna at, faucibus nisl. Nullam convallis sem eget enim convallis, vel sagittis leo hendrerit. Suspendisse hendrerit mauris faucibus, fermentum augue vel, pulvinar quam. Quisque commodo nulla vel nulla mattis porta. Maecenas fermentum dictum tempor. Praesent rutrum eu erat at euismod.".to_vec(); 
    data
}

#[wasm_bindgen_test]
fn buzhash_showcase() {
    let data = test_data();
    let len = data.len() as u32;
    log!("Input buffer size: {}", len);
    let mask: u32 = 8; // 11111111 comparable w/ min fastcdc setting
    let offsets = compute_chunks_buzhash(data, mask);
    let mut last_offset: u32 = 0;
    for offset in offsets {
        log!("Buzhash offsets start:{}, end:{}, len:{} bytes", last_offset, offset, offset - last_offset);
        last_offset = offset;
    }
    assert_eq!(len, last_offset);
}

#[wasm_bindgen_test]
fn buzhash_stability_test() {
    let data = test_data();
    let mask: u32 = 5; // 11111
    let offsets_1 = compute_chunks_buzhash(data, mask);
    let data_2 = test_data();
    let offsets_2 = compute_chunks_buzhash(data_2, mask);
    assert_eq!(offsets_1, offsets_2);
}


#[wasm_bindgen_test]
fn fastcdc_showcase() {
    let data = test_data();
    let len = data.len() as u32;
    log!("Input buffer size: {}", len);
    // min values from https://github.com/nlfiedler/fastcdc-rs/blob/300a86636201f1eda7b96d88aa1833cb774f606a/src/lib.rs#L20-L30
    let min_size: u32 = 64;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let offsets = compute_chunks_fastcdc(data, min_size, avg_size, max_size);
    let mut last_offset: u32 = 0;
    for offset in offsets {
        log!("Fastcdc offsets start:{}, end:{}, len:{} bytes", last_offset, offset, offset - last_offset);
        last_offset = offset;
    }
    assert_eq!(len, last_offset);
}

#[wasm_bindgen_test]
fn fastcdc_stability_test() {
    let data = test_data();
    // min values from https://github.com/nlfiedler/fastcdc-rs/blob/300a86636201f1eda7b96d88aa1833cb774f606a/src/lib.rs#L20-L30
    let min_size: u32 = 64;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let offsets_1 = compute_chunks_fastcdc(data, min_size, avg_size, max_size);
    let data_2 = test_data();
    let offsets_2 = compute_chunks_fastcdc(data_2, min_size, avg_size, max_size);
    assert_eq!(offsets_1, offsets_2);
}