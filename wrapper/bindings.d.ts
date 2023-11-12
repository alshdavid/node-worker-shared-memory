export type ID = number

export function js_string_new(): ID
export function js_string_get(id: ID): string
export function js_string_set(id: ID, update: string): void

export function js_number_new(): ID
export function js_number_get(id: ID): number
export function js_number_set(id: ID, update: number): void


export function js_vectors_new(): ID
export function js_vectors_push(id: ID, ref: ID): void
export function js_vectors_index(id: ID, index: number): void
export function js_vectors_len(): number

export function js_maps_new(): ID
export function js_maps_set(id: ID, key: string, ref: ID): void
export function js_maps_get(id: ID, key: string): ID
export function js_maps_remove(id: ID, key: string): boolean
export function js_maps_len(id: ID): number

export function js_typeof(id: ID): 'undefined' | 'string' | 'number' | 'vector' | 'map' | 'struct' | 'freed'
export function js_drop(id: ID): boolean
