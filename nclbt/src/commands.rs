use nlbl::{error::Result, util, BudgetCommand, BudgetCommands};

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
    pub account: String,
    pub create: bool,
    pub default_rename: Option<String>,
}

//TODO: take arg string directly from arg builder!!!! avoid hardcoded strings!!!
pub fn command_from_arg<'a>(
    arg: &str,
    mut vals: impl Iterator<Item = &'a String>,
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
            //BC::ClearExpense(vals.next().unwrap().into(), vals.next().cloned(), false)
            todo!("'clear' command")
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

pub fn parse_input(
    args: clap::ArgMatches,
) -> Result<(AppSettings, AccountOptions, BudgetCommands)> {
    //flags
    let mem_only: bool = args.get_flag("mem_only");
    let interactive_mode: bool = args.get_flag("interactive");
    let force: u8 = args.get_count("force");
    let dry_run: bool = args.get_flag("dry_run");
    //verbosity logic:
    //default to 1
    //each -v is +1
    //-q or -i is -1
    let verbosity: u8 =
        1 + args.get_count("verbose") - (interactive_mode || args.get_flag("quiet")) as u8;
    let json: bool = args.get_flag("json");

    let app_settings = AppSettings {
        mem_only,
        interactive_mode,
        dry_run,
        force,
        verbosity,
        json,
    };

    //account settings/commands
    let account: String = args
        .get_one("account")
        .or(args.get_one("new_account"))
        .cloned()
        .unwrap_or(String::from("default"));
    let create: bool = args.contains_id("new_account");
    let default_rename: Option<String> = args.get_one("default_name").cloned();

    let account_options = AccountOptions {
        account,
        create,
        default_rename,
    };

    let budget_commands: BudgetCommands = ["paycheck", "paid", "clear", "edit", "new", "pay"]
        .iter()
        .filter(|id| args.contains_id(id))
        .flat_map(|id| {
            args.get_occurrences(id).unwrap().filter_map(|arg_iter| {
                command_from_arg(id, arg_iter)
                    .map_err(|e| {
                        println!("{e}");
                        e
                    })
                    .ok()
            })
        })
        .collect::<BudgetCommands>();

    #[cfg(debug_assertions)]
    {
        println!("[DEV] parsed app settings: {app_settings:?}");
        println!("[DEV] parsed account options: {account_options:?}");
        println!("[DEV] parsed budget commands: {budget_commands:?}");
    }

    Ok((app_settings, account_options, budget_commands))
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
            account: "Demo User".into(),
            create: false,
            default_rename: None,
        },
        BudgetCommands::new(),
    )
}
