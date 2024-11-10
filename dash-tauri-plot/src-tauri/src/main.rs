use std::ffi::CString;
use std::mem::zeroed;
use std::ptr;
use tauri::{generate_context, Builder, Manager};
use winapi::um::processthreadsapi::CreateProcessA;
use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOA};
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn main() {
    Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let exe_path = app_handle
                .path()
                .resource_dir()
                .expect("Failed to locate resource directory")
                .join("app.exe");

            let exe_path_str = exe_path.to_str().expect("Failed to convert path to str");

            // Convert the exe path to a CString and bind to a variable to extend its lifetime
            let exe_path_cstring = CString::new(exe_path_str).expect("CString conversion failed");

            // Set up `STARTUPINFO` and `PROCESS_INFORMATION` structures
            let mut startup_info: STARTUPINFOA = unsafe { zeroed() };
            let mut process_info: PROCESS_INFORMATION = unsafe { zeroed() };
            startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

            // Use `CreateProcessA` to start `app.exe` without a console window
            let process_created = unsafe {
                CreateProcessA(
                    exe_path_cstring.as_ptr(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    0,
                    CREATE_NO_WINDOW,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &mut startup_info,
                    &mut process_info,
                )
            };

            if process_created == 1 {
                // Hide the window if it somehow appears
                unsafe { ShowWindow(process_info.hProcess as _, SW_HIDE) };
            } else {
                eprintln!("Failed to start app.exe");
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("Error while running Tauri application");
}
