// @ts-check
const { StringRef, NumberRef, VectorRef, MapRef, StructFactory, raw_api } = require('../rs-addon')

const map = new MapRef()

map.add('foo', NumberRef.from(42))
console.log(map.get('foo').obtainValue())


// const Thing = new StructFactory({
//   "key_1": "string",
//   "key_2": "number",
//   "key_3": "vector",
//   "key_4": "map",
//   "key_5": {
//     "key_1": "string"
//   }
// })

// const instance = Thing.new()

// let r1 = instance.getKey('key_1')
// let r2 = instance.getKey('key_2')

// const vec = new VectorRef()

// vec.push(new NumberRef())
// vec.push(new NumberRef())

// vec.index(0).drop()
// // vec.index(1).setValue(2)

// for (let i = 0; i < vec.len(); i++) {
//   console.log(vec.index(i))
// }
