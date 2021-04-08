# rust-node-perf  
Benchmarking of various frameworks used in interfacing rust with node.js. Many of the competing solutions arent clear on what approach is being used (FFI, N-API, WASM), which is a primary factor in the per-call performance overhead.

This project provides a minimal implementation of a function `add(a, b)` in each framework to benchmark these overheads so we can compare their performance.

# Requirements
- Node v8+
- rustup toolchain (rustc + cargo)
- wasm-buildpack ( `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` )
- Clone the repo and run `npm install`
## Results

┌─────────┬────────────┬────────────────────┬────────┐
│ (index) │   module   │    relativeTime    │ effort │
├─────────┼────────────┼────────────────────┼────────┤
│    0    │    'js'    │         1          │ lowest │
│    1    │ 'wasmPack' │ 1.7524918867705868 │ highest│
│    2    │   'neon'   │ 15.817699428654072 │ medium │
│    3    │  'napiRs'  │  3.13050560950176  │  low   │
└─────────┴────────────┴────────────────────┴────────┘
