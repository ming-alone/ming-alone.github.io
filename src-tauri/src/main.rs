/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-02-03 21:05:36
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-04 15:25:32
 * @FilePath: /tauri-app/src-tauri/src/main.rs
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager};
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}
// 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
// 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close).add_item(open));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
        match event.menu_item_id() {
            "quit" => {
            std::process::exit(0);
            }
            "close" => {
                event.window().emit("click", Payload { message: "12345".into()}).unwrap();
            // event.window().close().unwrap();
            }
            "open" =>{
                event.window().open_devtools();
            }
            _ => {}
        }
        })
        .setup(|app| {
      // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("click", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });
        // unlisten to the event using the `id` returned on the `listen_global` function
        // an `once_global` API is also exposed on the `App` struct
            app.unlisten(id);

      // emit the `event-name` event to all webview windows on the frontend
            app.emit_all("click", Payload { message: "Tauri is awesome11111!".into() }).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
