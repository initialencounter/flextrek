# Flextrek

## Description

A super-easy, windows-only crate to get focused explorer location or selected files path using hotkey!

## Usage

### Get selected files

[example/get_explorer_selected_file.rs](example/get_explorer_selected_file.rs)

```Rust
use flextrek::listen_selected_files;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("Start to listen explorer selected files");
    println!("Hotkey: {}", hotkey_str);
    listen_selected_files(hotkey_str, |files| async move {
        println!("Selected files: {:?}", files);
    })
    .await;
}
```

### Get focused explorer location

[example/get_explorer_location.rs](example/get_explorer_location.rs)

```Rust
use flextrek::listen_path;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("Start to listen explorer location");
    println!("Hotkey: {}", hotkey_str);
    let _ = listen_path(hotkey_str, |path| async move {
        println!("Current path: {:?}", path);
    })
    .await;
}
```
