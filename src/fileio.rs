use std::fs::{self, File};
use std::io;
use std::path;

//OS-dependent save location
const SAVE_DIR: &str = {
    #[cfg(target_os = "windows")]
    "%APPDATA%/.nclbt";
    #[cfg(target_os = "linux")]
    "$HOME/.local/share/nclbt"
};

//only returns a file if it exists, user must intentionally opt-in to making a new account
fn access_account_file(account: Option<&str>) -> Result<File, io::Error> {
    let account = account.unwrap_or("default");
    File::open(format!("{SAVE_DIR}{account}.bdgt"))
}
fn make_default_account_file() -> Result<File, io::Error> {
    make_account_file("default")
}
fn make_account_file(account: &str) -> Result<File, io::Error> {
    if fs::read_dir(SAVE_DIR).is_err() {
        fs::create_dir_all(
            path::absolute(SAVE_DIR)
                .unwrap_or_else(|_| panic!("SAVE_DIR constant invalid! {SAVE_DIR}")),
        )
        .unwrap();
    }
    File::create_new(format!("{SAVE_DIR}{account}.gv"))
}
