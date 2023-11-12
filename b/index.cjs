const { StringRef, NumberRef, VectorRef, ...native } = require('./api/index.cjs')

const vec = new VectorRef()

vec.push(new NumberRef())
vec.push(new NumberRef())

vec.index(0).setValue(2)
vec.index(1).setValue(2)

for (let i = 0; i < vec.len(); i++) {
  console.log(vec.index(i).obtainValue())
}