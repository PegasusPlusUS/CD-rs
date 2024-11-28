#![allow(nonstandard_style)]
use std::ptr::{null, null_mut};
use windows_sys::{
    core::{PCWSTR, PWSTR},
    Win32::{//Foundation::HANDLE,
        //Security::CreateProcessW,
        System::{Threading::{
                CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW, CREATE_UNICODE_ENVIRONMENT,
                },
            //Environment::GetEnvironmentStringsW,
        }
    },
};

mod spawn_with_pwd_per_drive;
use spawn_with_pwd_per_drive::create_child_process_with_pwd_per_drive;
fn main() {
    use std::collections::HashMap;

    // Create environment variables map
    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert("E_CURRENT_DIR".to_string(), r"E:\Study".to_string());
    env_vars.insert("D_CURRENT_DIR".to_string(), r"D:\Study".to_string());
    let result = create_child_process_with_pwd_per_drive("CMD.exe".to_string(), "-c 'DIR'".to_string(), ".".to_string(), &env_vars, false);
    println!("Create child process {}.", if result != 0 { "succeed" } else { "failed" });

    // Program to execute
    let application_name = "C:\\Windows\\System32\\notepad.exe";

    // Command line arguments (application name must also be included explicitly here)
    let _command_line = format!("{} {}", application_name, r"C:\test.txt");

    // Initialize structures
    let mut startup_info = STARTUPINFOW{cb:0,
        cbReserved2:0,
        dwFillAttribute:0,
        dwFlags:0,
        dwX:0,
        dwXCountChars:0,
        dwY:0,
        dwYCountChars:0,
        dwXSize:0,
        dwYSize:0,
        hStdError:null_mut(),
        hStdInput:null_mut(),
        hStdOutput:null_mut(),
        lpDesktop:null_mut(),
        lpReserved:null_mut(),
        lpReserved2:null_mut(),
        lpTitle:null_mut(),
        wShowWindow:0
    };
    let mut process_info = PROCESS_INFORMATION{dwProcessId:0, dwThreadId:0, hProcess:null_mut(), hThread:null_mut()};

    // Call CreateProcessW
    let success = unsafe {
        CreateProcessW(
            null(), //PCWSTR::from(application_name),
            null_mut(), //PWSTR::from(&command_line),
            null(),
            null(),
            0,
            CREATE_UNICODE_ENVIRONMENT,
            null(),    // Environment
            null(), //PCWSTR::from{"."},  // CWD
            &mut startup_info,
            &mut process_info,
        )
    };

    if success !=0 {
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
