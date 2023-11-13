const db = require('../rs-addon')

// const keypress = async () => {
//     process.stdin.setRawMode(true)
//     return new Promise(resolve => process.stdin.once('data', (data) => {
//         if (data.toString() === "q") {
//             process.exit(0)
//         }
//         process.stdin.setRawMode(false)
//         resolve()
//     }))
// }

// const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
// const charactersLength = characters.length;

// function makeid(length) {
//     let result = '';
//     let counter = 0;
//     while (counter < length) {
//       result += characters.charAt(Math.floor(Math.random() * charactersLength));
//       counter += 1;
//     }
//     return result;
// }


// const refs = new Map()

// for (let i = 0; i < 10_000; i++) {
//     const ref = new db.StringRef()
//     refs.set(ref.id, ref)
// }

// console.log('done')

// setTimeout(() => {}, 1_000_000)

// 120000000
// 112000000
// 122000000
// 212000000

// const LIMIT = 4_000_000_000

// let c_a = 0
// let c_b = 0
// const refs_a = []
// const refs_b = []

// // ;(async () => {
// // debugger

// while (true) {
//     if (c_a % 1_000_000 === 0) {
//         console.log(1, c_a)
//     }
//     if (c_a > LIMIT && ((c_a + c_b) % 1_000_000 === 0)) {
//         console.log(2, c_a + c_b)
//     }
//     if (c_a <= LIMIT) {
//         const ref = ''
//         refs_a.push(ref)
//         c_a++
//     } else 
//     if (c_a > LIMIT) {
//         const ref = ''
//         refs_b.push(ref)
//         c_b++
//     }
// }

// const LIMIT = 1_000_000 * 100

// let c_a = 0
// let c_b = 0
// const refs_a = []
// const refs_b = []

// // ;(async () => {
// // debugger

// while (true) {
//     if (c_a % 1_000_000 === 0) {
//         console.log(1, c_a)
//     }
//     if (c_a > LIMIT && ((c_a + c_b) % 1_000_000 === 0)) {
//         console.log(2, c_a + c_b)
//     }
//     if (c_a <= LIMIT) {
//         const ref = ''
//         refs_a.push(ref)
//         c_a++
//     } else 
//     if (c_a > LIMIT) {
//         const ref = ''
//         refs_b.push(ref)
//         c_b++
//     }
// }

// const LIMIT = 1_000_000 * 8

// let c_a = 0
// let c_b = 0
// let c_c = 0
// const refs_a = []
// const refs_b = []
// const refs_c = []

// // ;(async () => {
// // debugger

// while (true) {
//     if (c_a % 1_000_000 === 0) {
//         console.log(1, c_a)
//     }
//     if (c_a > LIMIT && ((c_a + c_b) % 1_000_000 === 0)) {
//         console.log(2, c_a + c_b)
//     }
//     if (c_a > LIMIT && c_b > LIMIT && ((c_a + c_b + c_c) % 1_000_000 === 0)) {
//         console.log(3, c_a + c_b + c_c)
//     }
//     if (c_a <= LIMIT) {
//         const ref = ''
//         refs_a.push(ref)
//         c_a++
//     } else 
//     if (c_a > LIMIT && c_b <= LIMIT) {
//         const ref = ''
//         refs_b.push(ref)
//         c_b++
//     } else 
//     if (c_c > LIMIT) {
//         const ref = ''
//         refs_c.push(ref)
//         c_c++
//     }
// }



// const LIMIT = 1_000_000 * 10
// let count = 0
// let curr = -1
// const refs = []

// while (true) {
//     if (count % LIMIT === 0) {
//         curr += 1
//         refs.push([])
//     }
//     if (count % 1_000_000 === 0) {
//         console.log(curr, count)
//     }
//     const ref = new Uint8Array(256)
//     refs[curr].push(ref)
//     count++
// }


// let c_a = 0
// const refs_a = []

// while (true) {
//     if (c_a % 1_000_000 === 0) {
//         console.log(1, c_a)
//     }
//     const ref = `${c_a}`
//     refs_a.push(ref)
//     c_a++
// }

// 59_000_000
// 63_000_000
// 112_000_000
// })()

// 10 loops @ 4gb kb @ 4 sec
// 10 loops @ 12.6gb @ 60 sec

// 1 vm snapshot 842mb
