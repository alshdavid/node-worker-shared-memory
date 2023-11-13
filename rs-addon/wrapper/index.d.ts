import { ID } from './native'

export * as raw_api from './native'

export class StringRef {
    constructor(id?: ID)
    readonly id: ID
    drop(): void
    setValue(update: string): void
    obtainValue(): string
}

export class NumberRef {
    constructor(id?: ID)
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

export class MapRef {
    constructor(id?: ID)
    readonly id: ID
    add(key: string, item: StringRef | NumberRef): void
    get(key: string): StringRef | NumberRef
    len(): number
    drop(): void
}

export type StructType = { [key: string]: 'string' | 'number' | 'vector' | 'map' | StructType }
export type StructTypeKey<T extends StructType, K extends keyof StructType> = T[K]
export type StructTypeRefMap = {
    'string': StringRef,
    'number': NumberRef,
    'vector': VectorRef,
    'map': MapRef,
}

export class StructFactory<T extends StructType> {
    constructor(shape: T)
    new(): StructRef<T>
}

export class StructRef<T extends StructType> {
    getKey<P extends string, U extends StructTypeKey<T, P>>(key: P): StructTypeRefMap[U]
    setKey<U extends keyof T>(key: U, value: any): void
    drop(): void
}
