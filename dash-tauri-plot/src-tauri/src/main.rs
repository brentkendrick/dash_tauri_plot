use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr;
use tauri::{generate_context, Builder, Manager};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::CREATE_NO_WINDOW;

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

            let exe_path_wide = to_wide_string(exe_path.to_str().expect("Failed to convert path"));

            unsafe {
                let mut startup_info: STARTUPINFOW = std::mem::zeroed();
                startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
                startup_info.dwFlags = CREATE_NO_WINDOW; // Set flag to suppress console

                let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

                let success = CreateProcessW(
                    ptr::null(),                        // No module name (use command line)
                    exe_path_wide.as_ptr() as *mut u16, // Command line
                    ptr::null_mut(),                    // Process handle not inheritable
                    ptr::null_mut(),                    // Thread handle not inheritable
                    0,                                  // Set handle inheritance to FALSE
                    CREATE_NO_WINDOW,                   // Suppress console window
                    ptr::null_mut(),                    // Use parent's environment block
                    ptr::null_mut(),                    // Use parent's starting directory
                    &mut startup_info,                  // Pointer to STARTUPINFO structure
                    &mut process_info,                  // Pointer to PROCESS_INFORMATION
                );

                if success == 0 {
                    eprintln!("Failed to start app.exe");
                } else {
                    // Close process and thread handles to avoid resource leaks
                    CloseHandle(process_info.hProcess);
                    CloseHandle(process_info.hThread);
                }
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
