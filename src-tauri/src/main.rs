/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-02-03 21:05:36
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-04 17:31:50
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
use rfd::FileDialog;
use std::fs;
use std::env;
use std::path::PathBuf;
// use std::io::Result;
use df_excel::{write::Excel, read::Read, Head};
use json::{array, object};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn open_file() {
    let files = FileDialog::new()
        .set_directory("/")
        .pick_file();
    // let text = fs::read_to_string(files.clone().unwrap().display().to_string()).unwrap();
    // println!("{}", text);
    // let path = format!("/Users/tears/Downloads/weixin/demo.xlsx");
    let path = format!("{}", files.unwrap().display().to_string());
    // let path = format!("{}/examples/file/a8df8a29b622d4017486f12df7586d3c6d662d80.xlsx", env!("CARGO_MANIFEST_DIR"));
    // let heads = vec![
    //     Head::new("order_no", "订单", "", 0),
    //     Head::new("code", "编号", "", 0),
    //     Head::new("name", "名称", "", 0),
    // ];
    let heads = vec![
        Head::new("order_no", "订单", "", 0),
        Head::new("code", "编号", "", 0),
        Head::new("name", "名称", "", 0),
        Head::new("item", "项号", "", 0),
        Head::new("barcode", "条码", "", 0),
        Head::new("code", "货号", "", 0),
        Head::new("name", "描述", "", 0),
        Head::new("qty", "数量", "", 0),
        Head::new("price", "单价", "", 0),
        Head::new("subtotal", "合计", "", 0),
    ];
    let data = Read::export(path.as_str(), 0, heads);
    println!("{:#}", data);
}

fn export() {
    if PathBuf::from(env!("CARGO_MANIFEST_DIR")) != env::current_dir().unwrap() {
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        env::set_current_dir(dir).unwrap();
    }
    let root_path = env::current_dir().unwrap();

    let web_conf_path = root_path.join("file");
    let web_conf_path = web_conf_path.join("name.xlsx");
    let web_conf_path = web_conf_path.to_str().unwrap();

    let mut excel = Excel::new("/Users/tears/Downloads/weixin/demo.xlsx");


    let heads = vec![
        Head::new("order_no", "订单", "", 0),
        Head::new("code", "编号", "", 0),
        Head::new("name", "名称", "", 0),
        Head::new("item", "项号", "", 0),
        Head::new("barcode", "条码", "", 0),
        Head::new("code", "货号", "", 0),
        Head::new("name", "描述", "", 0),
        Head::new("qty", "数量", "", 0),
        Head::new("price", "单价", "", 0),
        Head::new("subtotal", "合计", "", 0),
    ];

    let data = array![object! {"order_no":"123","code":"321","name":"订单123"}, object! {"order_no":"123345","code":"321222","name":"订单123123123"}];

    excel.set_page(0, "name0", heads.clone(), data.clone());
    excel.set_page(1, "name1", heads.clone(), data);

    excel.save();
}

// 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
// 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。

mod toy1 {
    // 方法1： 使用 include!
    include!("./libs/libs.rs");
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let open222 = CustomMenuItem::new("open222".to_string(), "Open222");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close).add_item(open).add_item(open222));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    export();
                    // std::process::exit(0);
                }
                "close" => {
                    toy1::run();
                    // event.window().emit("click", Payload { message: "12345".into() }).unwrap();
                    // event.window().close().unwrap();
                }
                "open" => {
                    event.window().open_devtools();
                }
                "open222" => {
                    open_file();
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
