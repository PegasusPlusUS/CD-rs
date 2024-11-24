// "D:." 
fn get_absolute_path(path_str: &str) -> Option<String> {
    use normalize_path::NormalizePath;
    let path = Path::new(path_str);
    let normalized_path = path.normalize();
    let path = normalized_path.to_str().unwrap();
    println!("Normalized path: {:?} {}", normalized_path, path);

    let drive_with_dot = format!("{}{}", path, if path.len() == 2 && path.chars().nth(1) == Some(':') { "." } else { "" });
    println!("Drive with dot {}, equals D:. {}", drive_with_dot, r"D:." == drive_with_dot);

    return get_full_path_name_w(&drive_with_dot);
}

fn get_full_path_name_w_stable(path_str: &str) -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::GetFullPathNameW;
    use winapi::um::winnt::WCHAR;

    const MAX_PATH: usize = 260;
    let mut buffer: [WCHAR; MAX_PATH] = [0; MAX_PATH];

    unsafe {
        // Convert input to wide string.
        let wide_path: Vec<u16> = OsString::from(path_str).encode_wide().chain(Some(0)).collect();
        let length = GetFullPathNameW(
            wide_path.as_ptr(),
            buffer.len() as u32,
            buffer.as_mut_ptr(),
            std::ptr::null_mut(),
        );

        // Check the result of GetFullPathNameW.
        if length > 0 && (length as usize) < MAX_PATH {
            // Convert wide string result back to Rust String.
            let path = OsString::from_wide(&buffer[..length as usize]);
            if let Some(path_str) = path.to_str() {
                return Some(path_str.to_string());
            }
        }

        // Log errors for debugging.
        // #[cfg(feature = "errhandlingapi")]
        // let error_code = winapi::um::errhandlingapi::GetLastError();
        // println!("GetFullPathNameW failed with error code: {}", error_code);
    }

    None
}

// GetFullPathW can't accept r"D:test", can only accept "D:." "D:\test"
fn get_full_path_name_w(path_str: &str) -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::GetFullPathNameW;
    use winapi::um::winnt::WCHAR;

    const MAX_PATH : usize = 260;
    let mut buffer: [WCHAR; MAX_PATH] = [0; MAX_PATH];

    unsafe {
        // Convert input to wide string.
        let wide_path: Vec<u16> = OsString::from(path_str).encode_wide().chain(Some(0)).collect();
        let length = GetFullPathNameW(
            wide_path.as_ptr(),
            buffer.len() as u32,
            buffer.as_mut_ptr(),
            std::ptr::null_mut(),
        );

        println!("GetFullPathNameW returns {}", length);

        if length > 0 && length as usize <= MAX_PATH {
            let path = OsString::from_wide(&buffer[..length as usize]);
            if let Some(path_str) = path.to_str() {
                println!("GetFullPathNameW returned path {}", path_str);
                let path_string = path_str.to_string();
                {
                    // If want to make dir, or create file, path_str may not exist
                    // use std::fs;
                    // if fs::metadata(path_str).is_ok() {
                        return Some(path_string);
                    // }
                }
            }
        }
    }

    None
}

fn try_get_absolute_path(path: &str) {
    println!("Try to get absolute path for {}", path);

    if let Some(current_dir) = get_absolute_path(path) {
        println!("Absolute path for {} is: {}", path, current_dir);
    } else {
        println!("Failed to get absolute path for {}", path);
    }

    use std::path::PathBuf;
    use omnipath::sys_absolute;
    if let Ok(current_dir) = sys_absolute(PathBuf::from(path).as_path()) {
        println!("Omnipath absolute path for {} is :{}", path, current_dir.display());
    }

}

fn print_cwd() {
    if let Ok(cwd) = std::env::current_dir() {
        println!("CWD is {:#?}", cwd.display());
    } else {
        println!("std::env::current_dir() failed");
    }
}

fn set_pwd(path: &str) {
    if let Ok(()) = std::env::set_current_dir(path) {
        println!("Set CWD as {}", path);
    } else {
        println!("Set CWD {} failed.", path);
    }
}

fn xmain() {
    // try_get_absolute_path(r"D:\test\vmm\..\..");
    // try_get_absolute_path(r"D:vmm\..");
    // try_get_absolute_path(r"D:x\..");
    // try_get_absolute_path(r"D:y\..");
    // try_get_absolute_path(r"D:z\..");
    // try_get_absolute_path(r"D:x.y.z\..");
    // try_get_absolute_path(r"D:vmm\x\../y/..\z\..");
    // try_get_absolute_path(r"D:vmm.tobemake\x\../y/..\z\..");
    // try_get_absolute_path(r"c:x\..");
    // try_get_absolute_path(r"d:y\..");
    // try_get_absolute_path(r"e:z\..");
    // try_get_absolute_path(r"c:x\..\test");
    // try_get_absolute_path(r"d:y\..\test");
    // try_get_absolute_path(r"e:z\..\test");
    set_pwd(r"C:\Users");
    try_get_absolute_path("c:test");
    print_cwd();
    set_pwd(r"D:\Project");
    env::set_var("C_CURRENT_DIR", r"C:\Users");
    try_get_absolute_path("d:test");
    print_cwd();
    set_pwd(r"E:\Study\nushell");
    env::set_var("D_CURRENT_DIR", r"D:\Project");
    print_cwd();
    try_get_absolute_path("c:test");
    try_get_absolute_path("d:test");
    try_get_absolute_path("e:test");
}

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
