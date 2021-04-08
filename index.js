const wasmPack = require('./wasm-pack/pkg')
const neon = require('./neon')
const napiRs = require('./napi-rs-addon')

const NS_PER_SEC = 1e9

const modules = {
  wasmPack,
  neon,
  napiRs
}

const pairs = Array(10000000).fill().map(() => [Math.random(), Math.random()])

function bench (func, jsTime) {
  let totalTime = 0

  for (const [a, b] of pairs) {
    const start = process.hrtime();
    func(a,b)
    const diff = process.hrtime(start)
    totalTime += diff[0] + (diff[1] / NS_PER_SEC)
  }

  return totalTime / jsTime
}

function add (a, b) {
  return a + b
}

const jsTime = bench(add, 1)

const results = [{ module: 'js', relativeTime: 1 , effort: 'high'}]

for (const mod of Object.keys(modules)) {
  results.push({ module: mod, relativeTime: bench(modules[mod].add, jsTime) , effort: 'low'})
}

console.table(results)
