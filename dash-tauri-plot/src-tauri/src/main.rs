use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("My Dash App").unwrap();

            // Launch executable without a console window
            Command::new("src-tauri/app.exe")
                .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
                .spawn()
                .expect("Failed to start app.exe");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
