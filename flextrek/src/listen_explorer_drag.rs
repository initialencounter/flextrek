//! 1. 独立线程安装 WH_MOUSE_LL 低级鼠标钩子（全局、免注入）
//! 2. 按住移动超过系统拖拽阈值（SM_CXDRAG/SM_CYDRAG）即判定拖拽开始，通过 PostThreadMessageW 唤醒消息泵
//! 3. 消息泵调用已有的 get_explorer_selected_file() 拿到被拖文件的完整路径，非空则回调
//! 4. DragHandle::unregister() 发 WM_QUIT 退出循环并 UnhookWindowsHookEx

use std::cell::Cell;
use std::sync::mpsc::channel;
use std::sync::{Arc, OnceLock};

use futures::Future;
use windows::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, GetSystemMetrics, PostThreadMessageW, SetWindowsHookExW,
    UnhookWindowsHookEx, MSG, MSLLHOOKSTRUCT, SM_CXDRAG, SM_CYDRAG, WH_MOUSE_LL, WM_APP,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_QUIT,
};

use crate::get_explorer_selected_file::get_explorer_selected_file;

const WM_FLEXTREK_DRAG: u32 = WM_APP + 1;

static THREAD_ID: OnceLock<u32> = OnceLock::new();

thread_local! {
    // 左键按下时光标位于 Explorer 窗口内，则记录起点；移动超过系统拖拽阈值视为开始拖拽
    static DRAG_START: Cell<Option<POINT>> = const { Cell::new(None) };
}

unsafe extern "system" fn low_level_mouse_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code == 0 {
        let msg = wparam.0 as u32;
        let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);
        match msg {
            WM_LBUTTONDOWN => {
                let pt: POINT = info.pt;
                DRAG_START.with(|c| c.set(Some(pt)));
            }
            WM_MOUSEMOVE => {
                DRAG_START.with(|c| {
                    if let Some(start) = c.get() {
                        let dx = (info.pt.x - start.x).abs();
                        let dy = (info.pt.y - start.y).abs();
                        if dx > GetSystemMetrics(SM_CXDRAG) || dy > GetSystemMetrics(SM_CYDRAG) {
                            c.set(None);
                            if let Some(&tid) = THREAD_ID.get() {
                                let _ =
                                    PostThreadMessageW(tid, WM_FLEXTREK_DRAG, WPARAM(0), LPARAM(0));
                            }
                        }
                    }
                });
            }
            WM_LBUTTONUP => DRAG_START.with(|c| c.set(None)),
            _ => {}
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub struct DragHandle {
    thread_id: u32,
}

impl DragHandle {
    pub fn unregister(self) {
        unsafe {
            let _ = PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
    }
}

/// 全局监听从资源管理器拖出文件/文件夹的手势（左键按下并移动超过系统拖拽阈值）。
/// 触发时读取拖拽源 Explorer 窗口的选中项（即被拖拽的文件/文件夹），非空时调用 callback。
/// 注意：全局同时只能有一个监听器；从桌面图标拖动暂不支持。
pub fn listen_explorer_drag_files<F, Fut>(callback: F) -> DragHandle
where
    F: Fn(Vec<String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
    let callback = Arc::new(callback);
    let (tx, rx) = channel::<u32>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle().clone();
        rt.block_on(async move {
            unsafe {
                let tid = GetCurrentThreadId();
                let _ = THREAD_ID.set(tid);
                let _ = tx.send(tid);

                let hook = match SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), None, 0)
                {
                    Ok(h) => h,
                    Err(e) => {
                        println!("Failed to set mouse hook: {:?}", e);
                        return;
                    }
                };

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    if msg.message != WM_FLEXTREK_DRAG {
                        continue;
                    }
                    // 耗时的 COM 查询移到阻塞线程池，消息泵线程绝不能阻塞：
                    // 低级钩子几秒内不被响应会被 Windows 静默移除。
                    let cb = callback.clone();
                    let blocking_handle = handle.clone();
                    handle.spawn_blocking(move || {
                        let files = get_explorer_selected_file();
                        if !files.is_empty() {
                            let _ = blocking_handle.block_on(cb(files));
                        }
                    });
                }

                let _ = UnhookWindowsHookEx(hook);
            }
        });
    });

    let thread_id = rx.recv().unwrap();
    DragHandle { thread_id }
}
