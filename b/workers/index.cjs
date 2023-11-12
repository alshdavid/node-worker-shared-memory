const { Worker } = require('worker_threads');

const THREADS = 2
const LOOP = 10
const hits = new Map()
let done = 0
let collisions = 0

for (let i = 0; i < THREADS; i++) {
  const worker = new Worker('./worker.cjs', { argv: [i, LOOP] });
  worker.addListener('message', (id) => {
    if (id === null) {
      done += 1 
      if (done === THREADS) {
        console.log(collisions)
      }
      return
    }
    console.log(`thread ${i}`, id)
    if (hits.has(id)) {
      console.log(`ALREADY HAS ${id}`)
      collisions += 1
    }
    hits.set(id, true)
  })
}
