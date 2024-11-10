use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::{generate_context, Builder, Manager};
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    Builder::default()
        .setup(|app| {
            // Obtain the AppHandle for accessing the resource directory
            let app_handle = app.app_handle();

            // Get the resource path for `app.exe`
            let exe_path = app_handle
                .path()
                .resource_dir()
                .expect("Failed to locate resource directory")
                .join("app.exe");

            Command::new(exe_path)
                .creation_flags(CREATE_NO_WINDOW) // Suppress console window
                .spawn()
                .expect("Failed to start app.exe");

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
