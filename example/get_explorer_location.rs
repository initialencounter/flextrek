// 在 explorer 通过快捷键获取当前路径
use flextrek::listen_path;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("开始监听 explorer 当前路径");
    println!("快捷键: {}", hotkey_str);
    let _ = listen_path(hotkey_str.to_string(), |path| async move {
        println!("当前路径: {:?}", path);
    })
    .await;
}
