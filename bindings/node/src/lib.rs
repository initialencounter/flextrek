#![deny(clippy::all)]

//! napi-rs v3 bindings for the `flextrek` crate.
//!
//! Every public API of the Rust crate is exposed to JavaScript:
//! - `getFocusedExplorerPath` / `getExplorerSelectedFile` (sync getters)
//! - `listen` / `listenPath` / `listenSelectedFiles` / `listenExplorerDragFiles` (hotkey & drag listeners)
//! - `parseHotkey` (hotkey string parser)
//!
//! The JS callbacks are invoked through `ThreadsafeFunction`s: the Rust side
//! registers the global hotkey / mouse hook on its own background thread and
//! marshals every event back to the Node.js main thread.

use std::path::PathBuf;
use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeCallContext, ThreadsafeFunctionCallMode};
use napi_derive::napi;

// ---------------------------------------------------------------------------
// Synchronous APIs
// ---------------------------------------------------------------------------

/// Returns the filesystem path of the focused Explorer window.
///
/// Returns an error when no Explorer window is focused.
#[napi]
pub fn get_focused_explorer_path() -> Result<String> {
  match flextrek::get_explorer_location::get_focused_explorer_path() {
    Ok(path) => Ok(path.to_string_lossy().into_owned()),
    Err(e) => Err(Error::new(Status::GenericFailure, e.to_string())),
  }
}

/// Returns the paths of the files/folders selected in the focused Explorer window.
#[napi]
pub fn get_explorer_selected_file() -> Vec<String> {
  flextrek::get_explorer_selected_file::get_explorer_selected_file()
}

/// The parsed modifier flags and virtual-key code of a hotkey string.
#[napi(object)]
pub struct HotkeyInfo {
  pub modifier: u32,
  pub key: u32,
}

/// Parses a hotkey string like `"Ctrl+Shift+z"` into its modifier flags and
/// virtual-key code, or `null` when the string is not a supported hotkey.
#[napi]
pub fn parse_hotkey(hotkey_str: String) -> Option<HotkeyInfo> {
  flextrek::hotkey::parse_hotkey(hotkey_str).map(|(modifier, key)| HotkeyInfo { modifier, key })
}

// ---------------------------------------------------------------------------
// Handles
// ---------------------------------------------------------------------------

/// Handle returned by `listen`, `listenPath` and `listenSelectedFiles`.
/// Call `unregister()` to stop listening and release the global hotkey.
#[napi]
pub struct HotkeyHandle {
  inner: Option<flextrek::HotkeyHandle>,
}

#[napi]
impl HotkeyHandle {
  #[napi]
  pub fn unregister(&mut self) {
    if let Some(inner) = self.inner.take() {
      inner.unregister();
    }
  }
}

impl Drop for HotkeyHandle {
  fn drop(&mut self) {
    if let Some(inner) = self.inner.take() {
      inner.unregister();
    }
  }
}

/// Handle returned by `listenExplorerDragFiles`.
/// Call `unregister()` to stop listening for file drags.
#[napi]
pub struct DragHandle {
  inner: Option<flextrek::listen_explorer_drag::DragHandle>,
}

#[napi]
impl DragHandle {
  #[napi]
  pub fn unregister(&mut self) {
    if let Some(inner) = self.inner.take() {
      inner.unregister();
    }
  }
}

impl Drop for DragHandle {
  fn drop(&mut self) {
    if let Some(inner) = self.inner.take() {
      inner.unregister();
    }
  }
}

// ---------------------------------------------------------------------------
// Listeners
// ---------------------------------------------------------------------------

/// Registers a global hotkey and calls `callback` (with no arguments) every
/// time it is pressed.
#[napi]
pub fn listen(hotkey: String, callback: Function<'static>) -> Result<HotkeyHandle> {
  let tsfn = Arc::new(
    callback
      .build_threadsafe_function::<()>()
      .build_callback(|_ctx: ThreadsafeCallContext<()>| Ok(()))?,
  );
  let tsfn_clone = tsfn.clone();
  let inner = flextrek::listen(hotkey, move || {
    let tsfn = tsfn_clone.clone();
    async move {
      tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
    }
  });
  Ok(HotkeyHandle { inner: Some(inner) })
}

/// Registers a global hotkey and calls `callback` with the focused Explorer
/// path every time it is pressed.
#[napi]
pub fn listen_path(hotkey: String, callback: Function<'static>) -> Result<HotkeyHandle> {
  let tsfn = Arc::new(
    callback
      .build_threadsafe_function::<PathBuf>()
      .build_callback(|ctx: ThreadsafeCallContext<PathBuf>| {
        Ok(ctx.value.to_string_lossy().into_owned())
      })?,
  );
  let tsfn_clone = tsfn.clone();
  let inner = flextrek::listen_path(hotkey, move |path: PathBuf| {
    let tsfn = tsfn_clone.clone();
    async move {
      tsfn.call(path, ThreadsafeFunctionCallMode::NonBlocking);
    }
  });
  Ok(HotkeyHandle { inner: Some(inner) })
}

/// Registers a global hotkey and calls `callback` with the selected
/// files/folders of the focused Explorer window every time it is pressed.
#[napi]
pub fn listen_selected_files(hotkey: String, callback: Function<'static>) -> Result<HotkeyHandle> {
  let tsfn = Arc::new(
    callback
      .build_threadsafe_function::<Vec<String>>()
      .build_callback(|ctx: ThreadsafeCallContext<Vec<String>>| Ok(ctx.value))?,
  );
  let tsfn_clone = tsfn.clone();
  let inner = flextrek::listen_selected_files(hotkey, move |files: Vec<String>| {
    let tsfn = tsfn_clone.clone();
    async move {
      tsfn.call(files, ThreadsafeFunctionCallMode::NonBlocking);
    }
  });
  Ok(HotkeyHandle { inner: Some(inner) })
}

/// Listens for files/folders dragged out of Explorer and calls `callback` with
/// the dragged items. Only one global listener can be active at a time.
#[napi]
pub fn listen_explorer_drag_files(callback: Function<'static>) -> Result<DragHandle> {
  let tsfn = Arc::new(
    callback
      .build_threadsafe_function::<Vec<String>>()
      .build_callback(|ctx: ThreadsafeCallContext<Vec<String>>| Ok(ctx.value))?,
  );
  let tsfn_clone = tsfn.clone();
  let inner =
    flextrek::listen_explorer_drag::listen_explorer_drag_files(move |files: Vec<String>| {
      let tsfn = tsfn_clone.clone();
      async move {
        tsfn.call(files, ThreadsafeFunctionCallMode::NonBlocking);
      }
    });
  Ok(DragHandle { inner: Some(inner) })
}
