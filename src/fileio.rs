use crate::budget;
use std::fs::File;

fn get_account_file(account: Option<&String>) -> Option<File> {
    None
}
fn make_account_file(account: &String) -> File {
    todo!()
}

pub fn load_bud(account: Option<&String>) -> budget::Budget{
    match get_account_file(account.clone()) {
        Some(file) => todo!(),
        None => budget::Budget::new(account)
    }
}
