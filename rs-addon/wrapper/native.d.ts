export type ID = number

export function js_string_new(): ID
export function js_string_get(id: ID): string
export function js_string_set(id: ID, update: string): void
export function js_string_delete(id: ID): void

export function js_number_new(): ID
export function js_number_get(id: ID): number
export function js_number_set(id: ID, update: number): void
export function js_number_delete(id: ID): void


export function js_vectors_new(): ID
export function js_vectors_push(id: ID, ref: ID): void
export function js_vectors_index(id: ID, index: number): void
export function js_vectors_len(): number

export function js_typeof(id: ID): 'undefined' | 'string' | 'number' | 'vector'
