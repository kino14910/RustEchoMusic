/**
 * 编译 plugins/packaged 下的所有第三方插件，并把产物放到插件根目录。
 *
 * loader 只按插件清单的 `entry` 找 `<entry>.dll`（连字符形式），而 cargo 产物
 * 里的 crate 名是下划线形式，位置也不一样（独立 workspace 在自身 target，
 * 根 workspace 成员在根 target）。每次手抄都容易漏，所以收进脚本。
 *
 * 用法：node scripts/build-packaged-plugins.mjs [--release]
 */
import { execFileSync } from 'node:child_process'
import { copyFileSync, existsSync, readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

const suffix =
    process.platform === 'win32' ? '.dll' : process.platform === 'darwin' ? '.dylib' : '.so'

const release = process.argv.includes('--release')
const profile = release ? 'release' : 'debug'

const root = process.cwd()
const packagedDir = join(root, 'plugins', 'packaged')
const rootTarget = join(root, 'target', profile)

if (!existsSync(packagedDir)) {
    console.error(`[plugins] no such directory: ${packagedDir}`)
    process.exit(1)
}

let built = 0
let failed = 0

for (const id of readdirSync(packagedDir)) {
    const dir = join(packagedDir, id)
    const manifestPath = join(dir, 'plugin.json')
    if (!existsSync(manifestPath)) continue

    let manifest
    try {
        manifest = JSON.parse(readFileSync(manifestPath, 'utf8'))
    } catch (error) {
        console.error(`[plugins] ${id}: invalid plugin.json (${error.message})`)
        failed += 1
        continue
    }

    const entry = typeof manifest.entry === 'string' ? manifest.entry.trim() : ''
    if (!entry) {
        console.error(`[plugins] ${id}: manifest has no 'entry'`)
        failed += 1
        continue
    }

    console.log(`[plugins] building ${id} (${profile})`)
    try {
        execFileSync('cargo', release ? ['build', '--release'] : ['build'], {
            cwd: dir,
            stdio: 'inherit',
        })
    } catch {
        console.error(`[plugins] ${id}: cargo build failed`)
        failed += 1
        continue
    }

    const artifact = `${entry.replace(/-/g, '_')}${suffix}`
    const target = `${entry}${suffix}`
    const candidates = [
        join(dir, 'target', profile, artifact),
        join(rootTarget, artifact),
    ]
    const source = candidates.find(candidate => existsSync(candidate))

    if (!source) {
        console.error(`[plugins] ${id}: built but '${artifact}' not found in:`)
        for (const candidate of candidates) console.error(`  - ${candidate}`)
        failed += 1
        continue
    }

    const destination = join(dir, target)
    try {
        copyFileSync(source, destination)
    } catch (error) {
        console.error(
            `[plugins] ${id}: cannot write ${destination} (${error.code ?? error.message})`,
        )
        if (error.code === 'EBUSY' || error.code === 'EPERM') {
            console.error('  the app is probably running and holding the library; close it and retry')
        }
        failed += 1
        continue
    }

    console.log(`[plugins] ${id}: ${source} -> ${destination}`)
    built += 1
}

console.log(`[plugins] done: ${built} deployed, ${failed} failed`)
if (failed > 0) process.exitCode = 1
