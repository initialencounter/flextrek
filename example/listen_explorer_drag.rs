use flextrek::listen_explorer_drag::listen_explorer_drag_files;

fn main() {
    println!("开始监听资源管理器的文件拖拽");
    let handle = listen_explorer_drag_files(|files| async move {
        println!("拖拽的文件: {:?}", files);
    });
    println!("30 秒后取消监听");
    std::thread::sleep(std::time::Duration::from_secs(30));
    println!("取消监听");
    handle.unregister();
}
