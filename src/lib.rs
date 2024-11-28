pub mod pwd_per_drive;
pub mod spawn_with_pwd_per_drive;

pub use pwd_per_drive::{print_cwd, set_pwd, try_get_absolute_path};
pub use spawn_with_pwd_per_drive::{create_child_process_with_pwd_per_drive};

fn main() {
    let mut assert_index = 0;
    let mut fail_count = 0;
    let mut failed_assertions: Vec<usize> = Vec::new();

    // Custom assert function using a closure
    let max_allowed_fail_cout = 0;
    let mut at_most_some_can_fail = |condition: bool| {
        if !condition {
            fail_count += 1;
            failed_assertions.push(assert_index);

            if fail_count > max_allowed_fail_cout {
                print!("Fail count exceed {}, the following asserts failed: ", max_allowed_fail_cout);
                for i in &failed_assertions {
                    print!("{}, ", i);
                };
                println!();
                assert!(false);
            }
        }
        assert_index += 1;
    };

    //at_most_some_can_fail(map.expand_path(PathBuf::from("c:").as_path()) == Some(PathBuf::from(r"C:\Users\Nushell\")));
}