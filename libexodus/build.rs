use std::env::current_dir;

fn main() {
    if let Ok(projectdir) = current_dir() {
        println!(
            "cargo:rustc-env=PROJECTDIR={}",
            projectdir.as_os_str().to_str().unwrap()
        );
    }
}
