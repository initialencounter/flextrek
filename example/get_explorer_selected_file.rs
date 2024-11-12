use flextrek::listen_selected_files;
fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("开始监听 explorer 选中的文件");
    println!("快捷键: {}", hotkey_str);
    let handle = listen_selected_files(hotkey_str.to_string(), |files| async move {
        println!("选中的文件: {:?}", files);
    });
    println!("10 秒后取消监听");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("取消监听");
    handle.unregister();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
