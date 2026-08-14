# flextrek-rs

> Windows-only Node.js bindings (built with [napi-rs](https://napi.rs)) for the [flextrek](https://github.com/initialencounter/flextrek) Rust crate.

Get the focused Explorer window's location or selected files/folders with a global hotkey, or listen for files being dragged out of Explorer — all from JavaScript, with no build toolchain required for consumers.

## Install

```bash
npm install flextrek-rs
```

> **Platform support:** currently `win32-x64-msvc` only.

## Requirements

- Windows (x64)
- Node.js `>= 12.22` (see the [`engines`](./package.json) field)

## Usage

### Get the focused Explorer path

```js
const { getFocusedExplorerPath } = require('flextrek-rs')

const path = getFocusedExplorerPath()
console.log(path) // e.g. 'C:\\Users\\me\\Documents'
```

Throws an error when no Explorer window is focused.

### Get selected files / folders

```js
const { getExplorerSelectedFile } = require('flextrek-rs')

const files = getExplorerSelectedFile()
console.log(files) // e.g. ['C:\\Users\\me\\Documents\\a.txt', 'C:\\Users\\me\\Documents\\folder']
```

Returns an empty array when nothing is selected in the focused Explorer window.

### Listen for a global hotkey

Register a hotkey and get a callback whenever it is pressed:

```js
const { listenSelectedFiles } = require('flextrek-rs')

const handle = listenSelectedFiles('Ctrl+Shift+z', (files) => {
  console.log('Selected files:', files)
})

// Stop listening and release the hotkey
handle.unregister()
```

Three hotkey listeners are available:

| Function | Callback payload |
| --- | --- |
| `listen(hotkey, cb)` | none (just a notification) |
| `listenPath(hotkey, cb)` | the focused Explorer path (`string`) |
| `listenSelectedFiles(hotkey, cb)` | the selected files/folders (`string[]`) |

### Listen for files dragged out of Explorer

```js
const { listenExplorerDragFiles } = require('flextrek-rs')

const handle = listenExplorerDragFiles((files) => {
  console.log('Dragged out of Explorer:', files)
})

// Stop listening
handle.unregister()
```

### Parse a hotkey string

```js
const { parseHotkey } = require('flextrek-rs')

parseHotkey('Ctrl+Shift+z') // { modifier: 6, key: 90 }
parseHotkey('not-a-hotkey') // null
```

## Hotkey format

Hotkeys are written as `Modifier+...Modifier+Key`, e.g. `Ctrl+Shift+z`, `Alt+Ctrl+s`.

**Modifiers** (any number, case-insensitive): `Ctrl`, `Alt`, `Shift`

**Keys** (case-insensitive):

- Letters: `A`–`Z`
- Digits: `0`–`9`
- Function keys: `F1`–`F12`
- Numpad: `NUMPAD0`–`NUMPAD9`, `NUMPADMULTIPLY`, `NUMPADADD`, `NUMPADSUBTRACT`, `NUMPADDIVIDE`, `NUMPADDECIMAL`, `NUMPADENTER`
- Other: `BACK`, `TAB`, `RETURN` / `ENTER`, `ESCAPE` / `ESC`, `SPACE`, `PRIOR`, `NEXT`, `END`, `HOME`, `LEFT`, `UP`, `RIGHT`, `DOWN`, `SELECT`, `PRINT`, `EXECUTE`, `SNAPSHOT`, `INSERT`, `DELETE`, `HELP`, `SCROLL`

The same format is accepted by `listen`, `listenPath`, `listenSelectedFiles` and `parseHotkey`.

## API

### `getExplorerSelectedFile(): string[]`

Returns the paths of the files/folders selected in the focused Explorer window.

### `getFocusedExplorerPath(): string`

Returns the filesystem path of the focused Explorer window. Throws when no Explorer window is focused.

### `parseHotkey(hotkeyStr: string): HotkeyInfo | null`

Parses a hotkey string into `{ modifier: number, key: number }` (Windows virtual-key / modifier flags), or `null` when the string is not a supported hotkey.

### `listen(hotkey: string, callback: () => void): HotkeyHandle`

Registers a global hotkey and calls `callback` every time it is pressed.

### `listenPath(hotkey: string, callback: (path: string) => void): HotkeyHandle`

Registers a global hotkey and calls `callback` with the focused Explorer path every time it is pressed.

### `listenSelectedFiles(hotkey: string, callback: (files: string[]) => void): HotkeyHandle`

Registers a global hotkey and calls `callback` with the selected files/folders of the focused Explorer window every time it is pressed.

### `listenExplorerDragFiles(callback: (files: string[]) => void): DragHandle`

Listens for files/folders dragged out of Explorer and calls `callback` with the dragged items.

### `HotkeyHandle` / `DragHandle`

Returned by the listener functions. Call `unregister()` to stop listening and release the global hotkey / drag hook.

## Limitations

- **Windows only** — currently `win32-x64-msvc`.
- The drag listener triggers when the left button is pressed on an Explorer window and the cursor moves beyond the system drag threshold; the dragged items are the Explorer selection at that moment.
- Only **one** global drag listener can be active at a time.
- Dragging from desktop icons is **not** supported.

## Development

Prerequisites: the latest stable [Rust](https://www.rust-lang.org/tools/install), [Node.js](https://nodejs.org), and [yarn](https://yarnpkg.com) (1.x).

```bash
yarn install   # install dependencies
yarn build     # compile the native addon (release)
yarn lint      # run oxlint + prettier + taplo checks
node simple-test.js  # smoke-test the binding
```

CI ([`.github/workflows/CI.yml`](../../.github/workflows/CI.yml)) builds the addon on `windows-latest` and publishes to npm when a version tag like `1.0.0` is pushed.

## License

[AGPL-3.0](./LICENSE)
