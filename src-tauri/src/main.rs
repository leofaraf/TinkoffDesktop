// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tauri::ipc::RemoteDomainAccessScope;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.ipc_scope().configure_remote_access(
                RemoteDomainAccessScope::new("www.tinkoff.ru")
                    .add_window("main")
                    .enable_tauri_api()
            );
            let window = app.get_window("main").expect("window not found");

            window.on("webview:load-commit", move |_| {
                // Вставьте JavaScript, который попытается перехватить и изменить поведение _blank
                window.eval(
                    r#"
                    (function() {
                        var links = document.getElementsByTagName('a');
                        for (var i = 0; i < links.length; i++) {
                            links[i].target = '_self';
                        }
                    })();
                    "#,
                ).unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
