use crate::error::Result;
use nlbl::{util, BudgetCommand, BudgetCommands};

#[derive(Debug)]
pub struct AppConfig {
    pub app_settings: AppSettings,
    pub account_options: AccountOptions,
    pub budget_commands: BudgetCommands,
}
#[derive(Debug)]
pub struct AppSettings {
    pub mem_only: bool,
    pub interactive_mode: bool,
    pub force: u8,
    pub dry_run: bool,
    pub verbosity: u8,
    pub json: bool,
}
#[derive(Debug)]
pub struct AccountOptions {
    pub account: Option<String>,
    pub create: bool,
    pub default_rename: Option<String>,
}

//TODO: take arg string directly from arg builder!!!! avoid hardcoded strings!!!
pub fn command_from_arg<'a>(
    arg: &str,
    mut vals: impl Iterator<Item = &'a String> + Clone,
) -> Result<BudgetCommand> {
    use BudgetCommand as BC;
    //TODO: manage unwraps!!
    Ok(match arg {
        "paycheck" => BC::SetPaycheck {
            amount: util::parse_dollar_string(vals.next().unwrap())?,
        },
        "paid" => {
            let amount = if let Some(v) = vals.next() {
                Some(util::parse_dollar_string(v)?)
            } else {
                None
            };
            BC::Paid { amount }
        }
        "clear" => {
            let mut inv = false;

            let targets = vals
                .inspect(|s| {
                    if s.starts_with("!") {
                        inv = true
                    }
                })
                //i hate that this is cloned()
                .cloned()
                .collect();
            BC::ClearExpense {
                targets,
                invert_selection: inv,
            }
        }
        "edit" => {
            let target: String = vals.next().unwrap().into();
            let mut new_name = None;
            let mut new_amount = None;
            vals.for_each(|v| {
                if let Ok(n) = util::parse_dollar_string(v) {
                    new_amount = Some(n);
                } else {
                    new_name = Some(v.clone());
                }
            });
            BC::EditExpense {
                target,
                new_name,
                new_amount,
            }
        }
        "new" => BC::NewExpense {
            name: vals.next().unwrap().into(),
            amount: util::parse_dollar_string(vals.next().unwrap())?,
        },
        "pay" => {
            let name = vals.next().unwrap().into();
            let amount = if let Some(v) = vals.next() {
                Some(util::parse_dollar_string(v)?)
            } else {
                None
            };
            BC::PayExpense { name, amount }
        }
        str => panic!("invalid BudgetCommand arg name !? {str}"),
    })
}

pub fn demo_defaults() -> (AppSettings, AccountOptions, BudgetCommands) {
    (
        AppSettings {
            mem_only: true,
            interactive_mode: true,
            force: 0,
            dry_run: false,
            verbosity: 1,
            json: false,
        },
        AccountOptions {
            account: Some("Demo User".into()),
            create: false,
            default_rename: None,
        },
        BudgetCommands::new(),
    )
}
