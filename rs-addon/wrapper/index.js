const native = require("./native");

function getRefFromId(id) {
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

class StringRef {
  id;

  constructor(
    id = native.js_string_new()
  ) {
    this.id = id;
  }

  static from(s) {
    const ref = new StringRef()
    ref.setValue(s)
    return ref
  }

  drop() {
    native.js_drop(this.id);
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

  static from(i) {
    const ref = new NumberRef()
    ref.setValue(i)
    return ref
  }

  drop() {
    native.js_drop(this.id);
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
    return getRefFromId(id)
  }

  len() {
    return native.js_vectors_len(this.id)
  }

  drop() {
    native.js_drop(this.id);
  }
}

class MapRef {
  id;

  constructor(
    id = native.js_maps_new()
  ) {
    this.id = id
  }

  add(key, item) {
    return native.js_maps_add(this.id, key, item.id)
  }
  
  get(key) {
    let id = native.js_maps_get(this.id, key)
    return getRefFromId(id)
  }
  
  remove(key) {
    return native.js_maps_remove(this.id, key)
  }
  
  drop() {
    native.js_drop(this.id);
  }

  len() {
    return native.js_maps_len(this.id)
  }
}

module.exports = {
    StringRef,
    NumberRef,
    VectorRef,
    MapRef,
    raw_api: native,
}