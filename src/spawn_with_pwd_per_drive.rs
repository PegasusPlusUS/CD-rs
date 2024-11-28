use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use windows_sys::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle},
        System::Threading::{
            CreateProcessW,
            CREATE_UNICODE_ENVIRONMENT,
            INFINITE,
            PROCESS_INFORMATION,
            STARTUPINFOW,
            WaitForSingleObject
        }
    }
};

pub fn create_child_process_with_pwd_per_drive(
    application: String,
    cmdline: String,
    pwd: String,
    env_vars: &HashMap<String, String>,
    wait_for_exit: bool
) -> i32 {
    let mut env_block: Vec<u16> = Vec::new();
    for (key, value) in env_vars {
        let mut entry: Vec<u16> = OsString::from(format!("{}={}", key, value)).encode_wide().collect();
        env_block.append(&mut entry);
        env_block.push(0);
        println!("Env:{}={}", key, value);
    }
    env_block.push(0);
    println!("Environment block size:{}", env_block.len());

    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    env_block.push(0); // Double null-terminate

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let application_wide: Vec<u16> = OsString::from(application).encode_wide().chain(Some(0)).collect();
    let cmdline_wide: Vec<u16> = OsString::from(cmdline).encode_wide().chain(Some(0)).collect();
    let pwd_wide: Vec<u16> = OsString::from(pwd).encode_wide().chain(Some(0)).collect();

    unsafe {
        let result:i32 =
        CreateProcessW(
            // PCWSTR::from(string_to_pcwstr(&application)), // App name
            // PWSTR::from(string_to_pwstr(&_cmdline)), // cmd line
            // null_mut(),
            // null_mut(),
            // 0,
            // CREATE_UNICODE_ENVIRONMENT,
            // env_block.as_ptr() as *const _,
            // PWSTR::from(string_to_pwstr(&pwd)),//PWSTR::from(string_to_pwstr(&r"C:\Windows\System32\")), // PWD
            // &mut si,
            // &mut pi,
            PCWSTR::from(application_wide.as_ptr()),
            PWSTR::from(cmdline_wide.as_ptr() as *mut _),
            null_mut(),
            null_mut(),
            0,
            CREATE_UNICODE_ENVIRONMENT,
            env_block.as_ptr() as *const _,
            PCWSTR::from(pwd_wide.as_ptr()),
            &mut si,
            &mut pi,
        );

        if result != 0 {
            println!("Process created successfully!");
            println!("Process ID: {} thread ID: {}", pi.dwProcessId, pi.dwThreadId);
    
            if wait_for_exit {
                WaitForSingleObject(pi.hProcess, INFINITE);
                println!("Process exits.");
            }
            // Close handles
            unsafe {
                CloseHandle(pi.hProcess);
                CloseHandle(pi.hThread);
            }
        }
        result
    }
}

