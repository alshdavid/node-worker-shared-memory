export type Primitive = String

export class String {
  id: string
  static new(): String
  static from(str: string): String
  transferValue(): string
  includes(str: string): boolean
}

export class Map<T extends Primitive> {
  static new<U extends Primitive>(): Map<U>
  get(key: string): T
  set(key: string, value: T): void
}

export type Syncable = Primitive | Map<any>
export type LockFn = () => void

export class Mutex<T extends Syncable> {
  static new<U extends Syncable>(value: U): Mutex<U>
  lock(): [LockFn, T]
  unlock(): void
}
