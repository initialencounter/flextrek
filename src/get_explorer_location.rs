use std::path::PathBuf;

use urlencoding::decode;
use windows::core::{w, Error, Interface, Result, BSTR};
use windows::Win32::Foundation::{HWND, SHANDLE_PTR};
use windows::Win32::System::Com::CoUninitialize;
use windows::Win32::{
    System::Com::{
        CoCreateInstance, CoInitializeEx, IDispatch, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED,
    },
    System::Variant::{VARIANT, VT_I4},
    UI::{
        Shell::{IShellWindows, IWebBrowser2, ShellWindows},
        WindowsAndMessaging::{FindWindowExW, GetForegroundWindow},
    },
};

pub fn get_focused_explorer_path() -> Result<PathBuf> {
    unsafe {
        // 检查 COM 初始化的结果
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(Error::new(
                windows::core::HRESULT(0),
                "Failed to initialize COM",
            ));
        }

        let hwnd_gfw: HWND = GetForegroundWindow();

        // 添加错误处理
        let shell_windows: IShellWindows =
            match CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER) {
                Ok(windows) => windows,
                Err(_e) => {
                    CoUninitialize();
                    return Err(Error::new(
                        windows::core::HRESULT(0),
                        "Failed to create ShellWindows instance",
                    ));
                }
            };

        // shell_windows 会在作用域结束时自动 drop

        // 添加错误处理
        let count = match shell_windows.Count() {
            Ok(c) => c,
            Err(_) => {
                CoUninitialize();
                return Err(Error::new(
                    windows::core::HRESULT(0),
                    "Failed to get window count",
                ));
            }
        };

        let result_hwnd = match FindWindowExW(Some(hwnd_gfw), None, w!("ShellTabWindowClass"), None)
        {
            Ok(hwnd) => hwnd,
            Err(_) => {
                CoUninitialize();
                return Err(Error::new(
                    windows::core::HRESULT(0),
                    "Failed to find explorer window",
                ));
            }
        };

        for i in 0..count {
            let mut variant = VARIANT::default();
            let inner = &mut *variant.Anonymous.Anonymous;
            inner.vt = VT_I4;
            inner.Anonymous.lVal = i;
            let item: IDispatch = match shell_windows.Item(&variant) {
                Ok(item) => item,
                Err(_) => continue,
            };

            let win: IWebBrowser2 = match item.cast() {
                Ok(win) => win,
                Err(_) => continue,
            };

            let current_hwnd: SHANDLE_PTR = match win.HWND() {
                Ok(hwnd) => hwnd,
                Err(_) => continue,
            };

            if hwnd_gfw.0 as i32 == current_hwnd.0 as i32
                || result_hwnd.0 as i32 == current_hwnd.0 as i32
            {
                let location_url: BSTR = match win.LocationURL() {
                    Ok(url) => url,
                    Err(_) => continue,
                };

                let url = location_url.to_string().replace("file:///", "");
                match decode(&url) {
                    Ok(decoded_url) => {
                        CoUninitialize();
                        return Ok(PathBuf::from(decoded_url.to_string()));
                    }
                    Err(_) => continue,
                }
            }
        }

        CoUninitialize();
        Err(Error::new(
            windows::core::HRESULT(0),
            "Failed to find explorer window or not focused",
        ))
    }
}
