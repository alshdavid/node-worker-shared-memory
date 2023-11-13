import { ID } from './bindings'

export * as raw_api from './bindings'

export class StringRef {
    constructor(id?: ID)
    readonly id: ID
    static from(i: string): StringRef
    drop(): void
    setValue(update: string): void
    obtainValue(): string
}

export class NumberRef {
    constructor(id?: ID)
    static from(i: number): NumberRef
    readonly id: ID
    drop(): void
    setValue(update: number): void
    obtainValue(): number
}

export class VectorRef {
    constructor(id?: ID)
    readonly id: ID
    push(item: StringRef | NumberRef): void
    index(index: number): StringRef | NumberRef
    len(): number
    drop(): void
}

export class MapRef<T extends StringRef | NumberRef | VectorRef | MapRef<any>> {
    constructor(id?: ID)
    readonly id: ID
    add(key: string, item: T): void
    get(key: string): T
    remove(key: string): void
    len(): number
    drop(): void
}

export type StructType = { [key: string]: 'string' | 'number' } // | 'vector' | 'map' } // | StructType }
export type StructTypeKey<T extends StructType, K extends keyof StructType> = T[K]
export type StructTypeRefMap = {
    'string': string,
    'number': number,
    // 'vector': VectorRef,
    // 'map': MapRef,
}

export type StructInitialValues<T extends StructType> = {
    [K in keyof T]: StructTypeRefMap[T[K]]
}

export class StructFactory<T extends StructType> {
    constructor(shape: T)
    new(initialValues?: StructInitialValues<T>): StructRef<T>
    new_proxy(initialValues?: StructInitialValues<T>): StructProxy<T>
}

export class StructRef<T extends StructType> {
    getKey<P extends string, U extends StructTypeKey<T, P>>(key: P): StructTypeRefMap[U]
    setKey<U extends keyof T>(key: U, value: any): void
    drop(): void
}

export type StructProxy<T extends StructType> = StructRef<T> & {
    [K in keyof T]: StructTypeRefMap[T[K]]
}