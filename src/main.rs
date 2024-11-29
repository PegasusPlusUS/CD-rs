#![allow(nonstandard_style)]
use std::ptr;
use windows_sys::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::HANDLE,
        //Security::CreteProcessW,
        System::{
            Threading::{
                CreateProcessW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTUPINFOW,
            },
            //Environment::GetEnvironmentStringsW,
        },
    },
};

fn main() {
    // Program to execute
    let application_name = "C:\\Windows\\System32\\notepad.exe";

    // Command line arguments (application name must also be included explicitly here)
    let command_line = format!("{} {}", application_name, r"C:\test.txt");

    // Initialize structures
    let mut startup_info = STARTUPINFOW;
    let mut process_info = PROCESS_INFORMATION;

    // Call CreateProcessW
    let success = unsafe {
        CreateProcessW(
            PCWSTR::from(application_name),
            PWSTR::from(&command_line),
            ptr::null(),
            ptr::null(),
            false,
            CREATE_UNICODE_ENVIRONMENT,
            ptr::null(),
            PCWSTR::default(),
            &mut startup_info,
            &mut process_info,
        )
    };

    if success.as_bool() {
        println!("Process created successfully!");
        println!("Process ID: {}", process_info.dwProcessId);

        // Close handles
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(process_info.hProcess);
            windows_sys::Win32::Foundation::CloseHandle(process_info.hThread);
        }
    } else {
        eprintln!("Failed to create process.");
    }
}
