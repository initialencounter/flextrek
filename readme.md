# Flextrek

## Description

A super-easy, windows-only crate to get focused explorer location or selected files path using hotkey, or listen for file drags out of explorer!

## Usage

### Listen for files dragged out of explorer

[example/listen_explorer_drag.rs](example/listen_explorer_drag.rs)

```Rust
use flextrek::listen_explorer_drag::listen_explorer_drag_files;

fn main() {
    println!("Start to listen file drags from explorer");
    let handle = listen_explorer_drag_files(|files| async move {
        println!("Dragged files: {:?}", files);
    });
    println!("30 seconds later, unregister");
    std::thread::sleep(std::time::Duration::from_secs(30));
    println!("Unregister");
    handle.unregister();
}
```

Notes:
- Triggers when the left button is pressed on an explorer window and the cursor moves beyond the system drag threshold; the dragged files are the explorer selection at that moment.
- Only one global listener at a time; dragging from desktop icons is not supported.

### Get selected files

[example/get_explorer_selected_file.rs](example/get_explorer_selected_file.rs)

```Rust
use flextrek::listen_selected_files;
fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("Start to listen explorer selected files");
    println!("Hotkey: {}", hotkey_str);
    let handle = listen_selected_files(hotkey_str.to_string(), |files| async move {
        println!("Selected files: {:?}", files);
    });
    println!("10 seconds later, unregister");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Unregister");
    handle.unregister();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

### Get focused explorer location

[example/get_explorer_location.rs](example/get_explorer_location.rs)

```Rust
use flextrek::listen_path;
fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("Start to listen explorer location");
    println!("Hotkey: {}", hotkey_str);
    let handle = listen_path(hotkey_str.to_string(), |path| move {
        println!("Current path: {:?}", path);
    });
    println!("10 seconds later, unregister");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Unregister");
    handle.unregister();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

## CHANGELOG

- v0.2.2: add listen_explorer_drag_files to listen for file drags out of explorer
- v0.2.1: remove async from listen_path and listen_selected_files
- v0.2.0: add unregister method
- v0.1.1: replace hotkey_str type from &str to String