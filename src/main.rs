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
}

fn main() {
    //try_get_absolute_path(r"D:");
    //try_get_absolute_path(r"D:vmm");
    for _ in 0..10 {
        try_get_absolute_path(r"C:\");
    }
    try_get_absolute_path(r"D:\test\vmm\..\..");
    try_get_absolute_path(r"D:vmm\..");
    try_get_absolute_path(r"D:x\..");
    try_get_absolute_path(r"D:y\..");
    try_get_absolute_path(r"D:z\..");
    try_get_absolute_path(r"D:x.y.z\..");
    try_get_absolute_path(r"D:vmm\x\../y/..\z\..");
    try_get_absolute_path(r"D:vmm.tobemake\x\../y/..\z\..");
}

use std::env;
use std::path::Path;

// CD by env
fn zmain() {
    // Store the current directory
    let original_dir = env::current_dir().expect("Failed to get current directory");
    // Change to the desired drive (e.g., D:)
    env::set_current_dir(Path::new(r"D:x\..\y\..\z\..\..")).expect("Failed to change directory");
    // Get the current directory on the D: drive
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory on D: drive: {:?}", current_dir);
    // Switch back to the original directoryD:
    env::set_current_dir(original_dir).expect("Failed to switch back to original directory");
}

