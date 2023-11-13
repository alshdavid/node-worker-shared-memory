const native = require('../../rs-addon/index.node')

module.exports = {
  js_string_new: native.js_string_new,
  js_string_get: native.js_string_get,
  js_string_set: native.js_string_set,
  js_string_delete: native.js_string_delete,
  js_number_new: native.js_number_new,
  js_number_get: native.js_number_get,
  js_number_set: native.js_number_set,
  js_number_delete: native.js_number_delete,
}

module.exports.StringRef = class StringRef {
  id

  constructor() {
    this.id = native.js_string_new()
  }

  static new() {
    return new StringRef(native.js_string_new())
  }

  drop() {
    native.js_string_delete(this.id)
  }

  setValue(update) {
    native.js_string_set(this.id, update)
  }

  obtainValue() {
    return native.js_string_get(this.id)
  }
}

module.exports.NumberRef = class NumberRef {
  id

  constructor() {
    this.id = native.js_number_new()
  }

  static new() {
    return new StringRef(native.js_number_new())
  }

  drop() {
    native.js_number_delete(this.id)
  }

  setValue(update) {
    native.js_number_set(this.id, update)
  }

  obtainValue() {
    return native.js_number_get(this.id)
  }
}

module.exports.VectorRef = class VectorRef {
  push(...ids) {

  }

  index(i) {}

  len() {
    
  }
}