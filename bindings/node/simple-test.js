const {
  getFocusedExplorerPath,
  getExplorerSelectedFile,
  parseHotkey,
  listen,
  listenPath,
  listenSelectedFiles,
  listenExplorerDragFiles,
} = require('./index')

for (const fn of [
  getFocusedExplorerPath,
  getExplorerSelectedFile,
  parseHotkey,
  listen,
  listenPath,
  listenSelectedFiles,
  listenExplorerDragFiles,
]) {
  if (typeof fn !== 'function') {
    throw new Error(`Expected export to be a function, got ${typeof fn}`)
  }
}

// parseHotkey: valid -> { modifier, key }, invalid -> null
const parsed = parseHotkey('Ctrl+Shift+z')
if (!parsed || parsed.modifier !== 0x0006 || parsed.key !== 0x5a) {
  throw new Error(`parseHotkey("Ctrl+Shift+z") returned unexpected result: ${JSON.stringify(parsed)}`)
}
if (parseHotkey('invalid') !== null) {
  throw new Error('parseHotkey should return null for an invalid hotkey string')
}

// getExplorerSelectedFile is safe to call with no Explorer focused: returns []
const selected = getExplorerSelectedFile()
if (!Array.isArray(selected)) {
  throw new Error(`getExplorerSelectedFile should return an array, got ${typeof selected}`)
}

console.log('Simple test passed');

(()=>{
  listenExplorerDragFiles((files) => {
    console.log('listenExplorerDragFiles callback called with files:', files);
  });
  setTimeout(() => {
    console.log('Stopping listenExplorerDragFiles after 90 seconds');
    process.exit(0);
  }, 90000);
})();