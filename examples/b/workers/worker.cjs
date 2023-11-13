const native = require('../../rs-addon/index.node')
const { parentPort } = require('worker_threads');

const sleep = (d = 0) => new Promise(res => setTimeout(res, d))

void async function main() {
  for (let i = 0; i < process.argv[3]; i++) {
    let id = native.js_string_new()
    parentPort.postMessage(id)
    await sleep()
  }
  parentPort.postMessage(null)
}()
