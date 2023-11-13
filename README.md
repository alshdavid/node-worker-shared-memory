# Node.js Shared Memory Across Workers

This npm package exposes a centralized storage for state shared between workers.

## Installation

```bash
npm install @alshdavid/worker-shared-memory
```

## Usage

```javascript
// main.js
import { StringRef, MapRef } from '@alshdavid/worker-shared-memory'
import { Worker } from 'node:worker_threads'

// Spawn worker
const worker = new Worker('./worker.js')

// Create Map in store
const map = new MapRef()
map.add('foo', StringRef.from("Hello World")) // TODO should be set()

// Give the worker the id of the Map
// I might add "named" values in future so you don't need to transfer IDs
worker.postMessage(map.id)
```

```javascript
// worker.js
import { MapRef } from '@alshdavid/worker-shared-memory'
import { parentPort } from 'node:worker_threads'

// Get the ID of the Map
parentPort.addEventListener('message', ({ data: map_id }) => {
    // Connect to the same map
    const map = new MapRef(map_id)

    console.log(map.get('foo').obtainValue()) // "Hello World"
})
```

# Types

```javascript
import { StringRef, NumberRef, MapRef, VectorRef } from '@alshdavid/worker-shared-memory'

const str = new StringRef() // Empty string (utf8 string)
const num = new NumberRef() // Empty number (64 bit float)
const map = new MapRef()    // Key/Value store that holds refs
const vec = new VectorRef() // Dynamic array that holds refs
```

## Structs

You can define and instantiate structs

```javascript
// Define struct
const Foo = new StructFactory({
    key_1: "string",
    key_2: "number",
})

const foo = Foo.new({
    key_1: "foo",
    key_2: 42,
})

console.log(foo.getKey('key_1')) // 'foo'
foo.setKey('key_1', 'something else')
```

You can access structs through a JavaScript proxy object for more ergonomic interactions. be mindful of accessing values as they are not references, the underlying value may change after it is obtained.

```javascript
const Foo = new StructFactory({
    key_1: "string",
    key_2: "number",
})

const foo = Foo.new_proxy({
    key_1: "foo",
    key_2: 42,
})

console.log(foo.key_1) // 'foo'
foo.key_1 = 'bar' 
console.log(foo.key_1) // 'bar'
```

## Safety and Memory usage

All values are stored in a shared heap managed by the native module. Interacting with values occurs behind synchronization controls like mutexes and atomics.

The shared heap is not subject to the maximum heap size assigned to Node.js that you can modify via `—-max-old-space-size` - however to ensure minimal memory usage within the Node.js process + threads, values are passed by reference identifiers.

```javascript
import { StringRef } from '@alshdavid/worker-shared-memory'

const str = StringRef.from('foo')

console.log(str)               // { id: 342 }
console.log(str.obtainValue()) // 'foo'
```

Only after a value is obtained is it copied into the Node.js heap and will, at that point, contribute to memory usage within the process/thread memory pool.

The limitation is how many references you can hold in the process - which appears to be several billion under Node's default memory limit.

## Manual Memory Management

Unfortunately, usage of this library requires the manual cleanup of values stored in the shared value store and does expose the consumer to use after free errors. 

Caution is advised, particularly with Vectors and Maps as dropping them does not drop their held resources. Structs drop their assigned values.

```javascript
import { StringRef } from '@alshdavid/worker-shared-memory'

const str = StringRef.from('foo')

console.log(str.obtainValue()) // 'foo'

str.drop() // will deallocate the value

console.log(str.obtainValue()) // Thrown Error ('use after free')
```
