use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use windows::Win32::UI::{
    Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS},
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

use crate::get_explorer_location::get_focused_explorer_path;
use crate::get_explorer_selected_file::get_explorer_selected_file;
use crate::hotkey::parse_hotkey;
use futures::Future;
use tokio::sync::oneshot;

pub mod get_explorer_location;
pub mod get_explorer_selected_file;
pub mod hotkey;

pub struct HotkeyHandle {
    cancel_sender: oneshot::Sender<()>,
}

impl HotkeyHandle {
    pub fn unregister(self) {
        let _ = self.cancel_sender.send(());
        unsafe {
            let _ = UnregisterHotKey(None, 1);
        }
    }
}

async fn listen_hotkey<F, Fut>(hotkey_str: String, mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = bool>,
{
    let hotkey_str_clone = hotkey_str.clone();
    if let Some((modifier, key)) = parse_hotkey(hotkey_str_clone) {
        unsafe {
            let _ = RegisterHotKey(None, 1, HOT_KEY_MODIFIERS(modifier), key);
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if msg.message == WM_HOTKEY && !callback().await {
                    break;
                }
            }
        }
    } else {
        println!("Invalid hotkey: {}", hotkey_str);
    }
}

pub fn listen_path<F, Fut>(hotkey_str: String, callback: F) -> HotkeyHandle
where
    F: Fn(PathBuf) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = ()> + Send,
{
    let (tx, rx) = oneshot::channel();
    let rx = Arc::new(Mutex::new(rx));

    let handle = HotkeyHandle { cancel_sender: tx };

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let callback = callback.clone();
            let rx = rx.clone();
            listen_hotkey(hotkey_str, move || {
                let callback = callback.clone();
                let rx = rx.clone();
                async move {
                    if rx.lock().unwrap().try_recv().is_ok() {
                        return false;
                    }
                    if let Ok(path) = get_focused_explorer_path() {
                        callback(path).await;
                    }
                    true
                }
            })
            .await;
        });
    });

    handle
}

pub fn listen_selected_files<F, Fut>(hotkey_str: String, callback: F) -> HotkeyHandle
where
    F: Fn(Vec<String>) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = ()> + Send,
{
    let (tx, rx) = oneshot::channel();
    let rx = Arc::new(Mutex::new(rx));

    let handle = HotkeyHandle { cancel_sender: tx };

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let callback = callback.clone();
            let rx = rx.clone();
            listen_hotkey(hotkey_str, move || {
                let callback = callback.clone();
                let rx = rx.clone();
                async move {
                    if rx.lock().unwrap().try_recv().is_ok() {
                        return false;
                    }
                    let file_list = get_explorer_selected_file();
                    callback(file_list).await;
                    true
                }
            })
            .await;
        });
    });

    handle
}
