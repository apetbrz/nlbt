use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::budget::Budget;

//version 1 with initial release
const SAVE_FORMAT_VERSION: u8 = 0;

#[derive(Serialize, Deserialize, Debug)]
struct SaveFormat {
    app: String,
    version: String,
    save_format: u8,
    data: Vec<u8>,
}
impl SaveFormat {
    fn new(account: &str) -> Self {
        SaveFormat {
            app: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            save_format: SAVE_FORMAT_VERSION,
            data: Self::budget_into_bytes(Budget::new(account)),
        }
    }
    fn save(budget: Budget) -> Self {
        SaveFormat {
            app: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            save_format: SAVE_FORMAT_VERSION,
            data: Self::budget_into_bytes(budget),
        }
    }
    fn into_bytes(self) -> Vec<u8> {
        bson::to_raw_document_buf(&self)
            .unwrap_or_else(|err| {
                panic!("SaveFormat struct failed to serialize to BSON doc, why? {err}\n{self:?}")
            })
            .into_bytes()
    }
    fn budget_into_bytes(budget: Budget) -> Vec<u8> {
        bson::to_raw_document_buf(&budget)
            .unwrap_or_else(|err| {
                panic!("Budget struct failed to serialize to BSON doc, why? {err}\n{budget:?}")
            })
            .into_bytes()
    }
}

//move active directory into application directory in system-dependent user data dir
pub fn relocate_to_application_dir() -> Result<()> {
    //get OS-dependent user data dir and append package folder
    let working_dir = dirs::data_local_dir()
        .unwrap_or_else(|| panic!("Unsupported Operating System"))
        .join(env!("CARGO_PKG_NAME"));

    #[cfg(debug_assertions)]
    println!("{working_dir:?}");

    //if the app directory does not exist, create it!
    if !working_dir.is_dir() {
        generate_application_dir(&working_dir)?
    }

    //move there
    Ok(std::env::set_current_dir(working_dir)?)
}

//creates application dir and subdirs
fn generate_application_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(fs::create_dir(path.join("data"))?)
}

//returns File for given account
fn get_account_file(account: &str) -> Result<File> {
    let file_name = format!("data/{account}.bson");

    //create file if not exists
    if !fs::exists(&file_name)? {
        make_new_account_file(account)?;
    }

    Ok(File::open(file_name)?)
}

//returns SaveFormat object from given account's save file, creating a new one if not present
//TODO: move account creation to confirmation dialogue
fn access_account_save_from_file(account: &str) -> Result<SaveFormat> {
    let mut file = get_account_file(account)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bson::from_slice(bytes.as_slice())?)
}

//take in an account name and return the appropriate Budget object
pub fn load_budget_account(account: Option<&str>) -> Result<Budget> {
    let account = account.unwrap_or("default");
    let save = access_account_save_from_file(account)?;

    Ok(bson::from_slice(&save.data)?)
}

//write a budget out to the account's save file
pub fn save_budget_to_account_file(budget: Budget) -> Result<()> {
    let account = &budget.account;
    let mut file = make_account_file(account)?;
    let data = SaveFormat::save(budget).into_bytes();
    file.set_len(0)?;
    file.write_all(data.as_slice())?;
    file.flush()?;
    Ok(())
}

//creates and returns a new file
pub fn make_new_account_file(account: &str) -> Result<()> {
    Ok(make_account_file(account)?.write_all(&SaveFormat::new(account).into_bytes())?)
}

//creates and returns an empty save file
pub fn make_account_file(account: &str) -> Result<File> {
    Ok(File::create(format!("data/{account}.bson"))?)
}
