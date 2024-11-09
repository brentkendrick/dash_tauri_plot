// Import the necessary modules from Tauri
use tauri::{self, generate_context, generate_handler, Builder};

// Example command function (modify or remove if not needed)
#[tauri::command]
fn my_custom_command() {
    println!("Custom command executed");
}

// Function to start the Tauri app
pub fn run() {
    Builder::default()
        .invoke_handler(generate_handler![my_custom_command]) // Register custom commands here
        .run(generate_context!())
        .expect("error while running Tauri application");
}
