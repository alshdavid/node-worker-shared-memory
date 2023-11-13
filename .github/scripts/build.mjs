// This script it terrible, it was just quick to write

import * as fs from 'node:fs'
import * as path from 'node:path'
import * as child_process from 'node:child_process'
import * as url from 'node:url';

const TARGET = process.argv[2] || 'package'
console.log(TARGET)

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const Paths = {
    Root: path.join(__dirname, '..', '..'),
    Github: path.join(__dirname, '..'),
    Scripts: path.join(__dirname),
    IndexNode: path.join(__dirname, '..', '..', 'index.node'),
    Dist: path.join(__dirname, '..', 'dist'),
    DistPackage: path.join(__dirname, '..', 'dist', 'package'),
}

function cargoBuild(...args) {
    if (fs.existsSync(Paths.IndexNode)) {
        fs.rmSync(Paths.IndexNode, {recursive: true})
    }
    child_process.execSync(`npx cargo-cp-artifact -nc index.node -- cargo build --message-format=json-render-diagnostics ${args.join(' ')}`, {
        stdio: 'inherit',
        cwd: Paths.Root
    })
}

function npmPack(output) {
    child_process.execSync(`pnpm pack --pack-destination ${path.join(Paths.Dist, 'packed')}`, {
        stdio: 'inherit',
        cwd: path.join(Paths.Dist, output)
    })
    // for (const filename of fs.readdirSync(path.join(Paths.Dist))) {
    //     if (filename.endsWith('.tgz')) {
    //         fs.cpSync(path.join(Paths.Dist, filename), path.join(Paths.Dist, `${output}.tgz`))
    //     }
    // }
}

if (fs.existsSync(Paths.Dist)) {
    fs.rmSync(Paths.Dist, {recursive: true})
}
fs.mkdirSync(Paths.Dist, { recursive: true })

const originalPackageJson = JSON.parse(fs.readFileSync(path.join(Paths.Root, 'package.json'), 'utf8'))

delete originalPackageJson.scripts
delete originalPackageJson.devDependencies
originalPackageJson.types = 'index.d.ts'
originalPackageJson.main = 'index.js'

// Root package
TARGET === 'package' && (() => {
    const packageJson = structuredClone(originalPackageJson)
    fs.cpSync(path.join(Paths.Root, 'wrapper'), path.join(Paths.DistPackage), {recursive : true})
    
    packageJson.name = `@alshdavid/${packageJson.name}`
    packageJson.optionalDependencies = {
        "@alshdavid/worker-shared-memory_linux-amd64": packageJson.version,
        "@alshdavid/worker-shared-memory_linux-arm64": packageJson.version,
        "@alshdavid/worker-shared-memory_macos-amd64": packageJson.version,
        "@alshdavid/worker-shared-memory_macos-arm64": packageJson.version,
        "@alshdavid/worker-shared-memory_windows-amd64": packageJson.version,
        "@alshdavid/worker-shared-memory_windows-arm64": packageJson.version,
    }

    fs.writeFileSync(path.join(Paths.DistPackage, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
})()

originalPackageJson.main = 'index.node'

// Linux amd64
TARGET === 'linux-amd64' && (() => {
    cargoBuild(
        '--release',
        '--target x86_64-unknown-linux-gnu',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["linux"]
    packageJson.cpu = ["x64"]
    packageJson.libc = ["glibc"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
    npmPack(TARGET)
})()

TARGET === 'linux-arm64' && (() => {
    cargoBuild(
        '--release',
        '--target aarch64-unknown-linux-gnu',
        '--config target.aarch64-unknown-linux-gnu.linker=\\"aarch64-linux-gnu-gcc\\"',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["linux"]
    packageJson.cpu = ["x64"]
    packageJson.libc = ["glibc"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
    npmPack(TARGET)
})()

TARGET === 'macos-amd64' && (() => {
    cargoBuild(
        '--release',
        '--target x86_64-apple-darwin',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["darwin"]
    packageJson.cpu = ["x64"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
    npmPack(TARGET)
})()

TARGET === 'macos-arm64' && (() => {
    cargoBuild(
        '--release',
        '--target aarch64-apple-darwin',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["darwin"]
    packageJson.cpu = ["arm64"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
    npmPack(TARGET)
})()

TARGET === 'windows-amd64' && (() => {
    cargoBuild(
        '--release',
        '--target x86_64-pc-windows-msvc',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["win32"]
    packageJson.cpu = ["x64"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
    npmPack(TARGET)
})()

TARGET === 'windows-arm64' && (() => {
    cargoBuild(
        '--release',
        '--target aarch64-pc-windows-msvc',
    )

    fs.mkdirSync(path.join(Paths.Dist, TARGET), { recursive: true })

    fs.cpSync(Paths.IndexNode, path.join(Paths.Dist, TARGET, 'index.node'), {recursive : true})
    fs.cpSync(path.join(Paths.Root, 'wrapper', 'bindings.d.ts'), path.join(Paths.Dist, TARGET, 'index.d.ts'), {recursive : true})

    const packageJson = structuredClone(originalPackageJson)
    
    packageJson.name = `@alshdavid/${packageJson.name}_${TARGET}`
    packageJson.os = ["win32"]
    packageJson.cpu = ["arm64"]
    
    fs.writeFileSync(path.join(Paths.Dist, TARGET, 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
})()

npmPack(TARGET)
