use std::ffi::CString;
use std::ptr;
use tauri::{generate_context, Builder, Manager};
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winuser::SW_HIDE;

fn main() {
    Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let exe_path = app_handle
                .path()
                .resource_dir()
                .expect("Failed to locate resource directory")
                .join("app.exe");

            let path = CString::new(exe_path.to_str().expect("Failed to convert path"))
                .expect("CString conversion failed");
            let operation = CString::new("open").expect("CString conversion failed");

            // Use ShellExecute to launch `app.exe` with the console hidden
            unsafe {
                ShellExecuteA(
                    ptr::null_mut(),
                    operation.as_ptr(),
                    path.as_ptr(),
                    ptr::null(),
                    ptr::null(),
                    SW_HIDE,
                );
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
