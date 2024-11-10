use std::process::Command;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Start the `app.exe` process
            Command::new("app.exe")
                .spawn()
                .expect("Failed to start app.exe");

            Ok(())
        })
        .run(tauri::generate_context!()) // No need to pass app_handle here
        .expect("Error while running Tauri application");
}
