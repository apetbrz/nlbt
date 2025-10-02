mod cli;
mod commands;
mod error;
mod fileio;

use error::{Error, Result};
use nlbl::*;

use crate::commands::AppConfig;

fn main() -> Result<()> {
    let cfg = cli::init_settings()?;

    fileio::relocate_to_working_dir()?;

    let loaded_budget = fileio::handle_account_load(&cfg)?;

    let worked_budget = match cfg.app_settings.interactive_mode {
        true => cli::run_interactive(loaded_budget.clone())?,
        false => execute_cmds(
            loaded_budget.clone(),
            cfg.budget_commands,
            cfg.app_settings.force,
        )?,
    };

    //output after processing
    if cfg.app_settings.json {
        todo!("JSON output")
    } else {
        match cfg.app_settings.verbosity {
            2.. => {
                println!("Old:\n{loaded_budget}\nNew:\n{worked_budget}")
            }
            1 => {
                println!("{worked_budget}")
            }
            0 => {}
        }
    }

    //save changes
    if !(cfg.app_settings.mem_only || cfg.app_settings.dry_run) {
        fileio::save_budget_to_account_file(worked_budget)?;
    }

    Ok(())
}
