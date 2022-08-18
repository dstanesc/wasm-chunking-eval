
## Wasm Content Defined Chunking Evaluation 

[Wasm](https://github.com/rustwasm/wasm-pack) library for [content-based slicing](https://en.wikipedia.org/wiki/Rolling_hash). Convenience wrapper on existing rolling hash implementations in rust, such as provided by [fastcdc](https://github.com/nlfiedler/fastcdc-rs), [asuran chunker](https://gitlab.com/asuran-rs/asuran), etc. 


## Build requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Test

```sh
wasm-pack test --headless --firefox
```

## Wasm build

```sh
wasm-pack build --out-dir pkg/webpack --out-name chunking --target bundler --scope dstanesc
wasm-pack build --out-dir pkg/node --out-name chunking --target nodejs --scope dstanesc
```

## Packing for local usage

```
npm pack pkg/node/
npm pack pkg/webpack/
```

## Install for NodeJS

```
npm install @dstanesc/wasm-chunking-node-eval
```

## Install for Webpack bundling

```
npm install @dstanesc/wasm-chunking-webpack-eval
```

## Usage

```js
// import {compute_chunks_buzhash, compute_chunks_fastcdc} from "@dstanesc/wasm-chunking-webpack-eval";
import {compute_chunks_buzhash, compute_chunks_fastcdc} from "@dstanesc/wasm-chunking-node-eval";

const buf = ...
// mask 0b11111111111111
const offsets_buz = compute_chunks_buzhash(buf, 15).values(); 
// chunk sizes: min 16 KiB, avg 32 KiB, max 64 KiB
const offsets_fast = compute_chunks_fastcdc(buf, 16384, 32768, 65536).values();   
```

## Licenses

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Fastcdc](https://lib.rs/crates/fastcdc)  is distributed under the terms of the  MIT license

[Asuran Chunker](https://lib.rs/crates/asuran-chunker) is distributed under the terms of the [BSD 2 Clause + Patent License](https://gitlab.com/asuran-rs/asuran/blob/381097b04ffb3c329d0fcdaa0965a8bd40592f2d/LICENSE).




