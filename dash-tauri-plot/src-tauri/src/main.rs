use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use tauri::{generate_context, Builder, Manager};
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let exe_path = app_handle
                .path()
                .resource_dir()
                .expect("Failed to locate resource directory")
                .join("app.exe");

            Command::new("powershell")
                .args(&[
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    &format!("Start-Process -FilePath '{}'", exe_path.display()),
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .expect("Failed to start app.exe via PowerShell");

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
