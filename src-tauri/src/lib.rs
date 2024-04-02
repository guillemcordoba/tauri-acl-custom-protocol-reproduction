use std::path::PathBuf;

use tauri::{http, ipc::CapabilityBuilder, utils::mime_type, Manager, Url, WebviewWindowBuilder};
use include_dir::Dir;
use include_dir::include_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .register_uri_scheme_protocol("hey", |_app, request| {
            // skip leading `/`
            let mut path = &request.uri().path()[1..];

            if path == "" {
                path = "index.html";
            }


            println!("path {:?}", path);
            if let Some(data) = include_dir!("../dist").get_file(path) {
                let data = data.contents_utf8().unwrap().as_bytes().to_vec();
                http::Response::builder().body(data).unwrap()
            } else {
                http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .header(
                        http::header::CONTENT_TYPE,
                        mime_type::MimeType::Txt.to_string(),
                    )
                    .body("failed to read file".as_bytes().to_vec())
                    .unwrap()
            }
        })
        .setup(|app| {
            let window = WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::CustomProtocol(Url::parse("hey://ho")?),
            )
            .build()?;

            app.add_capability(
                CapabilityBuilder::new("test")
                    .window("main")
                    .permission("dialog:allow-confirm"),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
