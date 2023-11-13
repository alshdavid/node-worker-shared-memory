const native = require("./native");

class StringRef {
  id;

  constructor(
    id = native.js_string_new()
  ) {
    this.id = id;
  }

  drop() {
    native.js_string_delete(this.id);
  }

  setValue(update) {
    native.js_string_set(this.id, update);
  }

  obtainValue() {
    return native.js_string_get(this.id);
  }
}

class NumberRef {
  id;

  constructor(
    id = native.js_number_new()
  ) {
    this.id = id;
  }

  drop() {
    native.js_number_delete(this.id);
  }

  setValue(update) {
    native.js_number_set(this.id, update);
  }

  obtainValue() {
    return native.js_number_get(this.id);
  }
}

class VectorRef {
  id;

  constructor(
    id = native.js_vectors_new()
  ) {
    this.id = id
  }

  push(...refs) {
    for (const ref of refs) {
      native.js_vectors_push(this.id, ref.id)
    }
  }

  index(index) {
    let id = native.js_vectors_index(this.id, index)
    let typeOf = native.js_typeof(id)
    if (typeOf === 'number') {
      return new NumberRef(id)
    }
    if (typeOf === 'string') {
      return new StringRef(id)
    }
    if (typeOf === 'vector') {
      return new VectorRef(id)
    }
    return 
  }

  len() {
    return native.js_vectors_len(this.id)
  }
}


module.exports = {
    StringRef,
    NumberRef,
    VectorRef,
    raw_api: native,
}