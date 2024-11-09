use tauri::{Builder, WindowBuilder};

fn main() {
    Builder::default()
        .setup(|app| {
            // Create the window
            let main_window = WindowBuilder::new(app, "main").title("Dash App").build()?;

            // Get the first Webview from the window and execute JavaScript
            if let Some(webview) = main_window.webviews().get(0) {
                webview.eval("window.location.href = 'http://127.0.0.1:8050';")?;
            }

            Ok(())
        })
        .run(tauri::generate_context!()) // Provide the context to run the app
        .expect("Error running Tauri application");
}
