use flextrek::listen_selected_files;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hotkey_str = "Ctrl+Shift+z";
    println!("开始监听 explorer 选中的文件");
    println!("快捷键: {}", hotkey_str);
    listen_selected_files(hotkey_str.to_string(), |files| async move {
        println!("选中的文件: {:?}", files);
    })
    .await;
}
