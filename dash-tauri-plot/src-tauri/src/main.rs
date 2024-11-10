use std::os::windows::process::CommandExt;
use std::process::Command;
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            Command::new("powershell")
                .args(&[
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    "launch_app_hidden.ps1",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .expect("Failed to start app.exe via PowerShell script");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
