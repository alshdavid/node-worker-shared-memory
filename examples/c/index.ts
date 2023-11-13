import { String, Map, Mutex } from './based';

const mutex = Mutex.new(String.new())

;(() => {
  const [lock, ref0] = mutex.lock()
  ref0.includes('hi')
  lock()
})()

// const ref0 = Map.new<String>();
// const ref1 = String.new();


// ref0.set("key", ref1);
