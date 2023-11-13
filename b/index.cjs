// @ts-check
const { StringRef, NumberRef, VectorRef, MapRef, StructFactory, raw_api } = require('../rs-addon')

const Thing = new StructFactory({
  "key_1": "string",
  "key_2": "number",
})

const instance = Thing.new_proxy({
  key_1: 'hello',
  key_2: 42,
})


console.log(instance.key_1)

instance.key_1 = 'foo'

console.log(instance.key_1)
