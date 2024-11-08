use std::path::PathBuf;

use windows::Win32::UI::{
    Input::KeyboardAndMouse::{HOT_KEY_MODIFIERS, RegisterHotKey},
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

use crate::get_explorer_location::get_focused_explorer_path;
use crate::get_explorer_selected_file::get_explorer_selected_file;
use crate::hotkey::parse_hotkey;
use futures::Future;

pub mod get_explorer_selected_file;
pub mod get_explorer_location;
pub mod hotkey;

async fn listen_hotkey<F, Fut>(hotkey_str: &'static str, mut callback: F)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = bool>,
{
    if let Some((modifier, key)) = parse_hotkey(hotkey_str) {
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

pub async fn listen_path<F, Fut>(
    hotkey_str: &'static str,
    callback: F,
) where
    F: Fn(PathBuf) -> Fut,
    Fut: Future<Output = ()>,
{
    listen_hotkey(hotkey_str, || async {
        if let Ok(path) = get_focused_explorer_path() {
            callback(path).await;
        }
        true
    }).await;
}

pub async fn listen_selected_files<F, Fut>(
    hotkey_str: &'static str,
    callback: F,
) where
    F: Fn(Vec<String>) -> Fut,
    Fut: Future<Output = ()>,
{
    listen_hotkey(hotkey_str, || async {
        let file_list = get_explorer_selected_file();
        callback(file_list).await;
        true
    }).await;
}
