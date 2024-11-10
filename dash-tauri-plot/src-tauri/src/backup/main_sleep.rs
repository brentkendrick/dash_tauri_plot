use std::process::{Command, Stdio};

fn main() {
    // Start the app.exe built with PyInstaller
    let app_process = Command::new("app.exe") // Replace with the path to your app.exe
        .stdout(Stdio::piped()) // Optional: Capture output if needed
        .spawn()
        .expect("Failed to start app.exe");

    // Ensure that the Dash app (via app.exe) is running before opening Tauri window
    std::thread::sleep(std::time::Duration::from_secs(20));

    // Launch Tauri with a window pointing to localhost:8050
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
