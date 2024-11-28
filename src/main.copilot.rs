use std::collections::HashMap;
use std::env;
use windows::Win32::System::Threading::CreateProcessW;

fn create_child_process(env_vars: &HashMap<String, String>) {
    let mut env_block: Vec<u16> = Vec::new();
    for (key, value) in env_vars {
        let mut entry: Vec<u16> = OsString::from(format!("{}={}", key, value)).encode_wide().collect();
        env_block.append(&mut entry);
        env_block.push(0);
    }
    env_block.push(0);

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    unsafe {
        CreateProcessW(
            PWSTR(null_mut()),
            PWSTR(null_mut()),
            null_mut(),
            null_mut(),
            false,
            0,
            env_block.as_ptr() as *const _,
            PWSTR(null_mut()),
            &mut si,
            &mut pi,
        );
    }
}

use std::os::windows::ffi::OsStrExt;

fn main() {
    // Create environment variables map
    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert("E_CURRENT_DIR".to_string(), current_dir_e);
    env_vars.insert("D_CURRENT_DIR".to_string(), current_dir_d);

    // Create child process with the environment block
    create_child_process(&env_vars);
}

use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HANDLE};
use windows::{
    CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW, PROCESS_CREATION_FLAGS,
};
use std::ptr::null_mut;

fn main1() {
    let application_name: PCWSTR = PCWSTR::from_raw("notepad.exe\0".encode_utf16().collect::<Vec<u16>>().as_ptr());
    let mut startup_info: STARTUPINFOW = STARTUPINFOW::default();
    let mut process_info: PROCESS_INFORMATION = PROCESS_INFORMATION::default();

    let result: BOOL = unsafe {
        CreateProcessW(
            application_name,
            null_mut(),
            null_mut(),
            null_mut(),
            false,
            PROCESS_CREATION_FLAGS(0),
            null_mut(),
            null_mut(),
            &startup_info,
            &mut process_info,
        )
    };

    if result.as_bool() {
        println!("Process created successfully!");
        // Use process_info to get information about the created process
    } else {
        println!("Failed to create process.");
    }
}

use std::ptr;
use windows::{
    core::PCWSTR,
    Win32::Foundation::{HANDLE, PWSTR},
    Win32::System::Threading::{
        CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW, CREATE_UNICODE_ENVIRONMENT,
    },
    Win32::System::Environment::GetEnvironmentStringsW,
};

fn main2() -> windows::core::Result<()> {
    // Program to execute
    let application_name = "C:\\Windows\\System32\\notepad.exe";

    // Command line arguments (application name must also be included explicitly here)
    let command_line = format!("{} {}", application_name, "C:\\test.txt");

    // Initialize structures
    let mut startup_info = STARTUPINFOW::default();
    let mut process_info = PROCESS_INFORMATION::default();

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
            windows::Win32::Foundation::CloseHandle(process_info.hProcess);
            windows::Win32::Foundation::CloseHandle(process_info.hThread);
        }
    } else {
        eprintln!("Failed to create process.");
    }

    Ok(())
}
