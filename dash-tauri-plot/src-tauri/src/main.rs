use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr;
use tauri::{generate_context, Builder, Manager};
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::SW_HIDE;

fn to_wide_string(value: &str) -> Vec<u16> {
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(Some(0))
        .collect()
}

fn main() {
    Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let exe_path: PathBuf = app_handle
                .path()
                .resource_dir()
                .expect("Failed to locate resource directory")
                .join("app.exe");

            let exe_path_str = exe_path.to_str().expect("Failed to convert path to str");

            // Convert the path to a wide string for ShellExecuteW
            let exe_path_wide = to_wide_string(exe_path_str);
            let operation = to_wide_string("open");

            unsafe {
                // Launch app.exe using ShellExecuteW with SW_HIDE to prevent any visible window
                ShellExecuteW(
                    ptr::null_mut(),                   // No specific window to launch from
                    operation.as_ptr() as LPCWSTR,     // "open" operation
                    exe_path_wide.as_ptr() as LPCWSTR, // Path to app.exe
                    ptr::null(),                       // No parameters
                    ptr::null(),                       // Default working directory
                    SW_HIDE,                           // Hide the window
                );
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
