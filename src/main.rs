use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::process::Command;

type AppResult<T> = Result<T, Box<dyn Error>>;

/// Run given `argv` and return its stdout.
fn get_stdout(argv: impl IntoIterator<Item = impl AsRef<OsStr>>) -> AppResult<String> {
    let mut argv_iter = argv.into_iter();
    Ok(String::from_utf8(
        Command::new(argv_iter.next().expect("We must have base process..."))
            .args(argv_iter)
            .output()?
            .stdout,
    )?)
}

fn main() -> AppResult<()> {
    if env::args().len() > 1 {
        return Err("I accept no arguments...".into());
    }

    let uname_stdout = get_stdout(["uname", "-r"])?;
    let booted_kernel = uname_stdout.replacen("-", ".", 1);

    let pacman_stdout = get_stdout(["pacman", "-Qe", "linux"])?;
    let installed_kernel = pacman_stdout
        .split(' ')
        .nth(1)
        .expect("pacman returned something weird");

    if booted_kernel == installed_kernel {
        println!("The installed kernel is also the one currently booted.")
    } else {
        println!("The installed kernel is newer than the currently booted! Consider rebooting.")
    }

    Ok(())
}
