use windows::{
    core::{Interface, VARIANT, w},
    Win32::{
        System::{
            Com::{
                CLSCTX_SERVER, CoCreateInstance, COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize,
                IDispatch, IServiceProvider,
            },
            SystemServices::{SFGAO_FILESYSTEM, SFGAO_FOLDER},
        },
        UI::{
            Shell::{
                IShellBrowser, IShellItemArray, IShellWindows, ShellWindows,
                SIGDN_DESKTOPABSOLUTEPARSING, SIGDN_FILESYSPATH, SVGIO_SELECTION,
            },
            WindowsAndMessaging,
        },
    },
};

pub fn get_explorer_selected_file() -> Vec<String> {
    unsafe {
        let mut file_list = Vec::new();

        // 检查 HRESULT
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            println!("Failed to initialize COM: {:?}", hr);
            return file_list;
        }

        let hwnd_gfw = WindowsAndMessaging::GetForegroundWindow();
        let shell_windows: IShellWindows =
            match CoCreateInstance(&ShellWindows, None, CLSCTX_SERVER) {
                Ok(windows) => windows,
                Err(e) => {
                    println!("Failed to create ShellWindows instance: {:?}", e);
                    return file_list;
                }
            };

        let result_hwnd = match WindowsAndMessaging::FindWindowExW(
            hwnd_gfw,
            None,
            w!("ShellTabWindowClass"),
            None,
        ) {
            Ok(hwnd) => hwnd,
            Err(e) => {
                println!("Failed to find ShellTabWindowClass window: {:?}", e);
                return file_list;
            }
        };

        let count = shell_windows.Count().unwrap_or_default();
        for i in 0..count {
            let variant = VARIANT::from(i);
            let window: IDispatch = match shell_windows.Item(&variant) {
                Ok(w) => w,
                Err(e) => {
                    println!("Failed to get window item: {:?}", e);
                    continue;
                }
            };

            let mut service_provider: Option<IServiceProvider> = None;
            // 检查 window.query 的 HRESULT
            let hr = window.query(
                &IServiceProvider::IID,
                &mut service_provider as *mut _ as *mut _,
            );
            if hr.is_err() {
                println!("Failed to query service provider: {:?}", hr);
                continue;
            }

            let service_provider = match service_provider {
                Some(sp) => sp,
                None => continue,
            };

            let shell_browser =
                match service_provider.QueryService::<IShellBrowser>(&IShellBrowser::IID) {
                    Ok(sb) => sb,
                    Err(e) => {
                        println!("Failed to query shell browser: {:?}", e);
                        continue;
                    }
                };

            let phwnd = match shell_browser.GetWindow() {
                Ok(hw) => hw,
                Err(e) => {
                    println!("Failed to get window handle: {:?}", e);
                    continue;
                }
            };

            if hwnd_gfw.0 != phwnd.0 && result_hwnd.0 != phwnd.0 {
                continue;
            }

            let shell_view = match shell_browser.QueryActiveShellView() {
                Ok(sv) => sv,
                Err(e) => {
                    println!("Failed to query active shell view: {:?}", e);
                    continue;
                }
            };

            let shell_items = match shell_view.GetItemObject::<IShellItemArray>(SVGIO_SELECTION) {
                Ok(si) => si,
                Err(e) => {
                    println!("Failed to get shell items: {:?}", e);
                    continue;
                }
            };

            let count = shell_items.GetCount().unwrap_or_default();
            for i in 0..count {
                let shell_item = match shell_items.GetItemAt(i) {
                    Ok(item) => item,
                    Err(e) => {
                        println!("Failed to get shell item at {}: {:?}", i, e);
                        continue;
                    }
                };

                // 如果不是文件对象则继续循环
                if let Ok(attrs) = shell_item.GetAttributes(SFGAO_FILESYSTEM) {
                    if attrs.0 == 0 {
                        continue;
                    }
                }

                // 如果不是文件则继续循环
                if let Ok(attrs) = shell_item.GetAttributes(SFGAO_FOLDER) {
                    if attrs.0 != 0 {
                        continue;
                    }
                }

                // 获取文件名
                if let Ok(display_name) = shell_item.GetDisplayName(SIGDN_FILESYSPATH) {
                    if let Ok(name) = display_name.to_string() {
                        file_list.push(name);
                        continue;
                    }
                }

                // 获取文件夹名
                if let Ok(display_name) = shell_item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING) {
                    if let Ok(name) = display_name.to_string() {
                        file_list.push(name);
                    }
                }
            }
            break;
        }
        CoUninitialize();
        file_list
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
