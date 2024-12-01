use pwd_per_drive::spawn_with_pwd_per_drive::create_child_process_with_pwd_per_drive;

fn main() {
    use std::collections::HashMap;

    // Create environment variables map
    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert("=E:".to_string(), r"E:\Study".to_string());
    env_vars.insert("=D:".to_string(), r"D:\Study".to_string());
    env_vars.insert("=C:".to_string(), r"C:\Users".to_string());
    let result = create_child_process_with_pwd_per_drive(
        //r"C:\Windows\System32\CMD.exe".to_string(),
        r"c:\Users\pegas\source\repos\PegasusPlusUS\Export\CD-rs\target\debug\show_pwd_per_drive.exe".to_string(),
        r"".to_string(),
        //r"-c 'echo Hello .'".to_string(),
        //r"C:\Windows\System32\".to_string(),
        r"c:\Users\pegas\source\repos\PegasusPlusUS\Export\CD-rs\target\debug\".to_string(),
        &env_vars,
        true
    );
    println!("Create show pwd process result:{}", result);
}