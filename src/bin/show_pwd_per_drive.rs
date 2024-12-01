use pwd_per_drive::{print_cwd, try_get_absolute_path};

fn list_pwd_per_drive(start_drive: char, end_drive: char)
{
    for drive in start_drive..=end_drive {
        let env_var = format!("={}:", drive);
        if let Ok(env_pwd) = std::env::var(env_var.clone()) {
            println!("Env(\"{}\")=\"{}\"", env_var, env_pwd);
        }
        try_get_absolute_path(&format!("{}:", drive));
    }
}

fn main() {
    print_cwd();

    list_pwd_per_drive('C', 'G');
}
