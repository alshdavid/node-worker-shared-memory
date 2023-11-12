const platform = process.platform
const arch = process.arch

try {
    if (platform === 'linux' && arch === 'x64') {
        module.exports = require('@alshdavid/worker-shared-memory_linux-amd64')
    }
    else if (platform === 'linux' && arch === 'arm64') {
        module.exports = require('@alshdavid/worker-shared-memory_linux-arm64')
    } 
    else if (platform === 'darwin' && arch === 'x64') {
        module.exports = require('@alshdavid/worker-shared-memory_macos-amd64')
    }
    else if (platform === 'darwin' && arch === 'arm64') {
        module.exports = require('@alshdavid/worker-shared-memory_macos-arm64')
    }
    else if (platform === 'win32' && arch === 'amd64') {
        module.exports = require('@alshdavid/worker-shared-memory_windows-amd64')
    }
    else if (platform === 'win32' && arch === 'arm64') {
        module.exports = require('@alshdavid/worker-shared-memory_windows-arm64')
    }
} catch (error) {
    module.exports = require('../index.node')
}
