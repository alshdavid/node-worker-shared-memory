import * as fs from 'node:fs'
import * as path from 'node:path'
import * as child_process from 'node:child_process'
import * as url from 'node:url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

child_process.execSync('pnpm run build', {
    stdio: 'inherit',
    cwd: path.join(path.normalize(__dirname, '..', '..'))
})
