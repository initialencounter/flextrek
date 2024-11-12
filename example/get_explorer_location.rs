// 在 explorer 通过快捷键获取当前路径
use flextrek::listen_path;
fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("开始监听 explorer 当前路径");
    println!("快捷键: {}", hotkey_str);
    let handle = listen_path(hotkey_str.to_string(), |path| async move {
        println!("当前路径: {:?}", path);
    });
    println!("10 秒后取消监听");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("取消监听");
    handle.unregister();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
